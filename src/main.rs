use std::io;


fn main() {
    

    loop {
    
    let mut return_temp = String::new();
    let mut selected_number = String::new();


 
     println!("Select 1 to convert from celcius to fahrenheit, or 2 for the inverse");
     
     io::stdin().read_line(&mut selected_number).expect("failed to read line!"); 

     let selected_number: i64  = selected_number.trim().parse().expect("please type a number!");

     if selected_number == 1 {

        println!("Enter temperature to convert (expected : celcius)");
       
        io::stdin().read_line(&mut return_temp).expect("failed to read line!");        

        let return_temp: f64 = return_temp.trim().parse().expect("please type a number!");
        
        let final_temp = converter(return_temp,selected_number); 
        
        println!("Temperature in fahrenheit: {final_temp}");
     } else if selected_number == 2{
        
        println!("Enter temperature to convert (expected : fahrenheit)");
        io::stdin().read_line(&mut return_temp).expect("failed to read line!");              
        let return_temp: f64 = return_temp.trim().parse().expect("please type a number!");
        let final_temp = converter(return_temp,selected_number); 
        println!("Temperature in fahrenheit: {final_temp}");

     }




   }
 
   
}

fn converter(temp: f64, inserted_number: i64) -> f64 {
   

   if inserted_number == 1 {

      (temp*1.8) + 32.0

   } else if inserted_number == 2 {

      (temp - 32.0)/1.8

   }else {
    
     1.0
   }
   
   
}





