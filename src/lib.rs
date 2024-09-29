use std::{collections::{HashMap, HashSet}, error::Error};

pub mod data;

use data::{CompactEmployee, CompactProject, Customer, Employee, Project, WeekData};
use reqwest::{header::{
    HeaderMap, HeaderValue, CACHE_CONTROL, CONTENT_TYPE, REFERER
}, Client, };

pub struct RoughlyRight {
    username: String,
    password: String,
    client: Client,
    logged_in: bool,
}

impl RoughlyRight {

    pub fn new(username: &str, password: &str) -> Self {

        // Build a client that can store cookies
        let client = Client::builder().cookie_store(true).build().unwrap(); 

        RoughlyRight { 
            username: username.to_string(), 
            password: password.to_string(),
            client,
            logged_in: false,
        }

    }

    pub async fn employees(&mut self) -> Result<Vec<Employee>, Box<dyn Error>> {

        self.login().await?;

        let url = "https://app.roughlyright.com/employees?active=true";
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            let project_list: Vec<Employee> = response.json().await?;
            return Ok(project_list);
        } else {
            eprintln!("Failed to fetch data: {}", response.status());
        }
        Ok(Vec::new())

    }

    pub async fn employees_map(&mut self) -> Result<HashMap<String, Employee>, Box<dyn Error>> {
        let list = self.employees().await?;
        let map: HashMap<String, Employee> = list.into_iter().map(|item| (item.id.clone(), item)).collect();
        Ok(map)
    }

    pub async fn projects(&mut self) -> Result<Vec<Project>, Box<dyn Error>> {

        self.login().await?;

        let url = "https://app.roughlyright.com/projects?finished=false&noGroupFilter=true&projection=planning";
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            let list: Vec<Project> = response.json().await?;
            return Ok(list);
        } else {
            eprintln!("Failed to fetch data: {}", response.status());
        }
        Ok(Vec::new())
    }

    pub async fn projects_map(&mut self) -> Result<HashMap<String, Project>, Box<dyn Error>> {
        let list = self.projects().await?;
        let map: HashMap<String, Project> = list.into_iter().map(|item| (item.id.clone(), item)).collect();
        Ok(map)
    }

    pub async fn customers(&mut self) -> Result<Vec<Customer>, Box<dyn Error>> {

        self.login().await?;

        let url = "https://app.roughlyright.com/customers";
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            let list: Vec<Customer> = response.json().await?;
            return Ok(list);
        } else {
            eprintln!("Failed to fetch data: {}", response.status());
        }
        Ok(Vec::new())

    }

    pub async fn customers_map(&mut self) -> Result<HashMap<String, Customer>, Box<dyn Error>> {
        let list = self.customers().await?;
        let map: HashMap<String, Customer> = list.into_iter().map(|customer| (customer.id.clone(), customer)).collect();
        Ok(map)
    }


    pub async fn week_hours(&mut self, week_start: &str, week_end: &str) -> Result<Vec<WeekData>, Box<dyn Error>> {

        self.login().await?;

        let url = format!("https://app.roughlyright.com/weekhours?allPlansForProjects=true&endWeek={}&startWeek={}", week_start, week_end);

        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let week_data_list: Vec<WeekData> = response.json().await?;
            return Ok(week_data_list);
        } else {
            // Handle the error
            eprintln!("Failed to fetch data: {}", response.status());
        }

        Ok(Vec::new())

    }

    pub async fn weekly_work(&mut self, week: &str) -> Result<HashMap<String, CompactProject>, Box<dyn Error>> {

        let week_list = self.week_hours(week, week).await?;
        let projects = self.projects_map().await?;
        let employees = self.employees_map().await?;
        let customers = self.customers_map().await?;

        let mut weekly_list: HashMap<String, CompactProject> = HashMap::new();

        for entry in week_list {
            if entry.project.is_some() {

                if entry.employee.is_none() || entry.project.is_none() {
                    continue;
                }


                if entry.weeks.is_none() {
                    continue;
                }

                let weeks = entry.weeks.unwrap();
                let current_week_hours = weeks.get(week).unwrap_or(&0.0);
                if *current_week_hours <= 0.0 {
                    continue;
                }

                let project = projects.get(entry.project.as_ref().unwrap());
                if project.is_none() {
                    continue;
                }
                let project = project.unwrap();


                if project.customer_id.is_none() {
                    println!("Customer not found: {:?} = {:?}", project.name, project.customer_id);
                    continue;
                }
                let customer = customers.get(project.customer_id.as_ref().unwrap());
                if customer.is_none() {
                    println!("Customer not found: {:?}", project.customer_id);
                    continue;
                }
                let customer = customer.unwrap();


                let employee = employees.get(entry.employee.as_ref().unwrap());
                if employee.is_none() {
                    continue;
                }
                let employee = employee.unwrap();


                let key = format!("{} - {}", customer.name, project.name);
                let key_clone = key.clone();
                let key_clone_2 = key.clone();

                if let std::collections::hash_map::Entry::Vacant(e) = weekly_list.entry(key) {
                    let mut set: HashSet<CompactEmployee> = HashSet::new();
                    let person = CompactEmployee {
                        name: employee.name.clone(),
                        image: employee.image.clone(),
                    };
                    set.insert(person);
                    let compact_project = CompactProject {
                        project: key_clone,
                        employees: set,
                    };
                    e.insert(compact_project);
                } else {
                    let list = weekly_list.get_mut(&key_clone_2).unwrap();
                    let person = CompactEmployee {
                        name: employee.name.clone(),
                        image: employee.image.clone(),
                    };
                    list.employees.insert(person);
                }

            }

        }

        Ok(weekly_list)

    }

    pub async fn login(&mut self) -> Result<(), Box<dyn Error>> {

        if self.logged_in {
            return Ok(());
        }

        let url = "https://app.roughlyright.com/auth/local";

        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        headers.insert(REFERER, HeaderValue::from_static("https://app.roughlyright.com/rr/login"));

        let body = format!("username={}&password={}", self.username, self.password);

        let response = self.client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
        .await?;

        let status = response.status();

        if status.is_success() {
            self.logged_in = true;
            return Ok(());
        } else {
            eprintln!("Failed to login: {}", status);
        }

        Ok(())

    }
}

