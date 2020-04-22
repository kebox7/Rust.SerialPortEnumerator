#![allow(non_snake_case)]

extern crate serde;
extern crate wmi;

use serde::Deserialize;
use wmi::{ COMLibrary, WMIConnection };
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Eq)]
struct SerialPortInfo {
    Name: String,
    Description: String,
}

impl PartialEq for SerialPortInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
    }
}

impl Ord for SerialPortInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        let retval = self.Name.chars().count().cmp(&other.Name.chars().count());
        if retval == Ordering::Equal { self.Name.cmp(&other.Name) } else { retval }
    }
}

impl PartialOrd for SerialPortInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Deserialize)]
    struct Win32SerialPort {
        DeviceID: String,
        Description: String,
    }

    #[derive(Deserialize)]
    struct Win32PnPEntity {
        Caption: String,
    }

    let now = Instant::now();
    let comLibrary = COMLibrary::new()?;
    let wmiConnection = WMIConnection::new(comLibrary.into())?;

    let mut result: Vec<SerialPortInfo> = Vec::new();
    /*result.push(SerialPortInfo {
        Name: "COM11".to_string(),
        Description: "Тест 11".to_string(),
    });
    result.push(SerialPortInfo {
        Name: "COM10".to_string(),
        Description: "Тест 10.2".to_string(),
    });
    result.push(SerialPortInfo {
        Name: "COM10".to_string(),
        Description: "Тест 10.1".to_string(),
    });*/

    let itemsWin32SerialPort: Vec<Win32SerialPort> = wmiConnection.raw_query("SELECT * FROM Win32_SerialPort")?;
    for item in itemsWin32SerialPort {
        result.push(SerialPortInfo {
            Name: item.DeviceID.clone(),
            Description: item.Description.clone()
        });
        //println!("{:5} {}", item.DeviceID, item.Description);
    }
    //println!();

    let itemsWin32PnPEntity: Vec<Win32PnPEntity> = wmiConnection.raw_query("SELECT * FROM Win32_PnPEntity WHERE PNPClass = 'Ports' AND Caption LIKE '%COM%' AND ConfigManagerErrorCode = 0")?;
    for item in itemsWin32PnPEntity {
        //retain
        let index = item.Caption.rfind("(COM");
        if index != None {
            let pos = index.unwrap();
            result.push(SerialPortInfo {
                Name: item.Caption[pos..].trim_matches(|c| c == '(' || c == ')').to_string(),
                Description: item.Caption[..pos].trim().to_string()
            });
        }
        //println!("{}", item.Caption);
    }
    //println!();

    result.sort();
    result.dedup();
    let elapsed = now.elapsed();

    println!("Name   Description");
    println!("----   -----------");
    for item in result {
        println!("{:5}  {}", item.Name, item.Description);
    }
    println!();

    println!("Execution time, seconds: {:.6}", elapsed.as_secs_f64());

    Ok(())
}
