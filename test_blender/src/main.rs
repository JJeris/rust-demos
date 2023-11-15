// use std::process::{Command, ExitStatus};
// use tokio::main;

// async fn run_blender(blender_launcher_path: &str) -> Result<bool, &str> {
//     // let launch_config_path: &str = "../src/python_scripts/launch_config.py"; // if Production, use "_up_/src/python_scripts/show_version.py" 
//     let _child_process = std::process::Command::new(blender_launcher_path)
//         .args(&[
//             // "--python", 
//             // &launch_config_path,
//             "-con", 
//             ]) //-con, "--background", If Dev environment, use "../src/python_scripts/show_version.py", 
        
        
//         .spawn()
//         .expect("Failed to execute the .exe file");

//     println!("Executable launched");
//     return Ok(true);
// }

// async fn run_blender(blender_launcher_path: &str, script_name: &str, script_args: &[&str]) -> Result<bool, String> {
//     let _child_process = Command::new(blender_launcher_path)
//         .args(&["-P", script_name])
//         .args(script_args)
//         .spawn()
//         .expect("Failed to execute the .exe file");

//     println!("Executable launched");
//     Ok(true)
// }

// #[tokio::main]
// async fn async_main() {
//     let script_name = "C:\\Users\\J\\Desktop\\PA\\code\\rust-webscraping\\test_blender\\src\\test.py";
//     let script_args = vec!["arg1", "arg2"]; // Replace with your actual arguments

//     let result = run_blender(
//         "C:\\blenderbaseapps\\daily\\blender-4.1.0-alpha+main.0746435cdd89-windows.amd64-release\\blender-launcher.exe",
//         script_name,
//         &script_args,
//     ).await.unwrap();
//     println!("{result}");
// }

// fn main() {
//     // Run the tokio runtime and start the asynchronous main function
//     async_main();
// }



// async fn run_blender(blender_launcher_path: &str, script_name: &str, script_args: &[&str]) -> Result<bool, String> {
//     let mut child_process = Command::new(blender_launcher_path)
//         .args(&["-P", script_name])
//         .args(script_args)
//         .spawn()
//         .map_err(|e| format!("Failed to execute the .exe file: {}", e))?;

//     println!("Executable launched");

//     // Wait for the Blender process to complete
//     let status: ExitStatus = child_process
//         .wait()
//         .map_err(|e| format!("Failed to wait for the child process: {}", e))?;

//     if status.success() {
//         Ok(true)
//     } else {
//         Err(format!("Blender process exited with an error: {:?}", status))
//     }
// }

// #[tokio::main]
// async fn async_main() {
//     let script_name = "C:\\Users\\J\\Desktop\\PA\\code\\rust-webscraping\\test_blender\\src\\test.py";
//     let script_args = vec!["arg1", "arg2"]; // Replace with your actual arguments

//     let result = run_blender(
//         "C:\\blenderbaseapps\\daily\\blender-4.1.0-alpha+main.0746435cdd89-windows.amd64-release\\blender-launcher.exe",
//         script_name,
//         &script_args,
//     ).await;

//     match result {
//         Ok(true) => println!("Blender process completed successfully"),
//         Err(err) => eprintln!("Error: {}", err),
//         _ => {}
//     }
// }

// fn main() {
//     // Run the tokio runtime and start the asynchronous main function
//     // tokio::runtime::Runtime::new().unwrap().block_on(async_main());
//     async_main();
// }




use std::process::Command;

//??? WOOOOOOOOOOOOOOOOOOOOOOOORKS
// fn main() {
//     // Path to Blender executable
//     let blender_executable = "C:\\blenderbaseapps\\daily\\blender-4.1.0-alpha+main.0746435cdd89-windows.amd64-release\\blender-launcher.exe";

//     // Path to the Python script
//     let python_script = "C:\\Users\\J\\Desktop\\PA\\code\\rust-webscraping\\test_blender\\src\\test.py";

//     // Variable to pass to the Python script
//     let variable_to_pass = "John";

//     // Build the command to run Blender with the Python script and the variable
//     let output = Command::new(blender_executable)
//         .arg("-con")  
//         .arg("--python")            // Run a Python script
//         .arg(python_script)   // Path to the Python script
//         .arg("--")            // Separator for Blender arguments and script arguments
//         .arg(variable_to_pass) // Variable to pass to the Python script
//         .output()
//         .expect("Failed to start Blender");

//     // Print the output of Blender (stdout and stderr)
//     println!("Blender output:\n{}", String::from_utf8_lossy(&output.stdout));
//     eprintln!("Blender error output:\n{}", String::from_utf8_lossy(&output.stderr));

//     // Check if Blender was launched successfully
//     if output.status.success() {
//         println!("Blender started successfully");
//     } else {
//         eprintln!("Error starting Blender");
//     }
// }

// ?? THIS WOOOOOOOOOOOOOOOOOOOOOORKS
fn launch_blender(blender_launcher_path: &str, value_to_calculate: f64) {
    /*******************************************
    Launches Blender passing it a python scripts thats reads blender_config/blender_config.json.

    Args:
        blender_launcher_path: &str - path to the blender-launher.exe

    Author: @JJeris

    Notes: Need to make it async, although work pretty well as it is.
    *******************************************/
    let python_script = format!(
        r#"
import math
def main(data):
    input_value = {}
    result = math.sqrt(input_value)
    print(f'The square root of {{input_value}} is: {{result}}')
    print(data)
main(90)
"#,
        value_to_calculate
    );

// print(f'The square root of ' + input_value +' is: '+ result)
    let _child_process = std::process::Command::new(blender_launcher_path)
        .args(&[
            "--python-expr", 
            python_script.as_str(),
            "-con", 
            ]) 
        
        
        .spawn()
        .expect("Failed to execute the .exe file");

    println!("Executable launched");
    // return Ok(true);

}


fn main() {
    let value_to_calculate = 25.0; // You can change this to any desired value
    launch_blender("C:\\blenderbaseapps\\daily\\blender-4.1.0-alpha+main.0746435cdd89-windows.amd64-release\\blender-launcher.exe", value_to_calculate);
}


    // let launch_config_path: &str = "../src/python_scripts/launch_config.py"; // if Production, use "_up_/src/python_scripts/show_version.py" 