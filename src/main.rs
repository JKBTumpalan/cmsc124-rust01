use std::io;

// Structures
struct Product {
    product_id : u32 ,
    product_name : String ,
    price : f64 ,
    quantity : u32
}
struct Customer {
    name : String ,
    cart : Vec < String > ,
    total_cost : f64
}

// Menu display
fn display_menu(){
    println!("[1] Add a Product");
    println!("[2] Buy a Product");
    println!("[3] Edit a Product");
    println!("[4] Delete a Product");
    println!("[5] View all Products");
    println!("[6] View all Customers");
    println!("[7] Exit");
    println!("==========================");
}

// Checker for uniqueness of ID
fn check_id(product_data: &mut Vec<Product>, id:u32) -> bool {
    if product_data.len() == 0{
        return true;
    }
    for item in product_data {
        if item.product_id == id{
            return false;
        }
    }

    return true;
}

// ID input for products, uses the function above for checking the ID's uniqueness
fn get_unique_id(product_data: &mut Vec<Product>) -> u32{
    loop {
        let mut id = String::new();
        println!("Enter product ID: ");
        io::stdin().read_line(&mut id).expect("Error"); //wait for user input
        let id:u32 = id.trim().parse().expect("Error");

        // If unique, return u32 ID.
        if check_id(product_data, id) == true{
            return id;
        }
        // If not unique, print prompt.
        println!("\n [!] Product ID already exists [!]\n");
    }
}

// Deletes a product in the product vector
fn delete_product(product_data: &mut Vec<Product>, id:u32){
    // If there are no products, there are nothing to delete.
    if product_data.len() == 0{
        println!("\n[!] Product list is empty, cannot delete anything. [!] \n");
    }
    // Scan all products
    for item in 0..(product_data.len()){
        // If the ID is found, delete using the remove function
        if product_data[item].product_id == id{
            println!("\n[!] Product {} deleted [!] \n", product_data[item].product_id);
            product_data.remove(item);
            break;
        }
        // If the ID is not found and the iterator is already in the end of the list, print prompt
        else if (item == (product_data.len()-1)) && (product_data[item].product_id != id){
            println!("\n[!] Product id not found [!] \n");
        }
    }
}

fn edit_product(product_data: &mut Vec<Product>, id:u32){
    // If there are no products yet, there are nothing to buy.
    if product_data.len() == 0{
        println!("\n[!] Product list is empty, cannot edit anything.[!]\n");
    }
    // Scan all items in the product vector
    for item in 0..(product_data.len()){
        // If the given product ID is found,
        if product_data[item].product_id == id{
            // Initialize price and quantity variables
            let mut price = String::new();
            let mut quantity = String::new();
            
            // Ask for new values
            println!("Enter new product price: ");
            io::stdin().read_line(&mut price).expect("Error"); //wait for user input
            let price:f64 = price.trim().parse().expect("Error");
        
            println!("Enter new product quantity: ");
            io::stdin().read_line(&mut quantity).expect("Error"); //wait for user input
            let quantity:u32 = quantity.trim().parse().expect("Error");
            
            // update values of the Product Object in the Product vector.
            product_data[item].price = price;
            product_data[item].quantity = quantity;

            // Prompt
            println!("\n[!] Product succesfully edited [!] \n");

            break;
        }
        else if (item == (product_data.len()-1)) && (product_data[item].product_id != id){
            // If the iterator is in the end of the list and there are still no matching ID, print prompt
            println!("\n[!] Product id not found [!] \n");
        }
    }
}

fn edit_customer(customer_data: &mut Vec<Customer>, product_data: &mut Vec<Product>, customer_name: &mut String){
    if product_data.len() == 0{
        println!("\n[!] Product list is empty, cannot buy anything. [!] \n");
    } else {
        // Show products to choose from if there are products to show
        show_products(product_data);
    }

    // Ask for the Product ID
    let mut to_buy = String::new();
    println!("Enter product ID to buy: ");
    io::stdin().read_line(&mut to_buy).expect("Error"); //wait for user input
    let to_buy:u32 = to_buy.trim().parse().expect("Error");

    //Search product ID in the product vector, then return the index of the matching object
    let mut product_index = product_data.len();

    for index in 0..product_data.len(){
        if product_data[index].product_id == to_buy{
            product_index = index;
        }
    }

    // If the product index did not change, print not found prompt
    if product_index == product_data.len(){
        println!("\n[!] Product not found [!] \n");
    } else {
        // Else, if found, check for product quantity. If greater than zero,
        if product_data[product_index].quantity > 0{
            for customer in customer_data{
                // Search for the name of the customer, then update his/her cart.
                if customer.name.eq(customer_name){
                    // Update customer cart, push product_name string to its cart vector,
                    customer.cart.push((&product_data[product_index].product_name).to_string());
                    customer.total_cost = customer.total_cost + product_data[product_index].price;
                    // Update product object in the product vector, decrement quantity by 1
                    product_data[product_index].quantity = product_data[product_index].quantity - 1;
                    println!("\n[!] Product successfully bought [!]\n");
                    break;
                }
            }
        } else {
            // If product quantity is 0, print prompt
            println!("\n[!] Product is out of stock [!]\n");
        }
    }

}

fn add_product(product_data: &mut Vec<Product>){
    // Define variables for storing user input
    let mut name = String::new();
    let mut price = String::new();
    let mut quantity = String::new();

    // Get unique ID usign the functions above
    let id = get_unique_id(product_data);

    // Get user input, parse to their respective data types if applicable.
    println!("Enter product name: ");
    io::stdin().read_line(&mut name).expect("Error");

    println!("Enter product price: ");
    io::stdin().read_line(&mut price).expect("Error");
    let price:f64 = price.trim().parse().expect("Error");

    println!("Enter product quantity: ");
    io::stdin().read_line(&mut quantity).expect("Error");
    let quantity:u32 = quantity.trim().parse().expect("Error");

    // Define new product object using the values acquired from the user.
    let new_product = Product {
        product_id: id,
        product_name: name,
        price: price,
        quantity: quantity
    };

    // Print success prompt
    println!("=====================================");
    println!("[!] New product added to the shop [!]");
    println!("{} - {} : {}PHP ({} pcs)", &new_product.product_id, &new_product.product_name[0..(&new_product.product_name.len()-1)], &new_product.price, &new_product.quantity);
    println!("=====================================");

    // Push new product object to the product vector.
    product_data.push(new_product);
}

fn add_customer(customer_data: &mut Vec<Customer>, customer_name: &mut String){
    let mut flag = true;

    // Define new customer object using the values of the passed parameters
    let new_customer = Customer {
        name: (&customer_name).to_string(),
        cart: Vec::new(),
        total_cost: 0.0, //Initial spent is zero
    };

    // Seearch for customers
    for cust in 0..customer_data.len(){
        // If customer name is found in the customer vector, print prompt and set push flag to zero.
        if customer_data[cust].name.eq(&new_customer.name){
            println!("Customer is already in the database.");
            flag = false;
            break;
        } else if cust == customer_data.len()-1 && customer_data[cust].name.ne(&new_customer.name){
            // If the customer name is not yet in the database, print added prompt
            println!("=====================================");
            println!("[!] New customer added to the shop [!]");
            println!("{}", new_customer.name);
            println!("=====================================");
        }
    }

    // Push customer to the customer vector, using the flags set above
    if flag{
        customer_data.push(new_customer);
    }
}

// Print product vector
fn show_products(product_data: &mut Vec<Product>){
    println!("============ PRODUCTS =============");
    if product_data.len() == 0{
        println!("[!] There are no products available [!]")
    } else {
        for item in 0..(product_data.len()) {
            println!("{} - {} : {}PHP ({} pcs)", &product_data[item].product_id, &product_data[item].product_name[0..(&product_data[item].product_name.len()-1)], &product_data[item].price, &product_data[item].quantity);
            println!("|")
        }
    }
    println!("===================================");
}

// Print customer vector
fn show_customers(customer_data: &mut Vec<Customer>){
    println!("============ CUSTOMERS =============");
    if customer_data.len() == 0{
        println!("[!] There are no customers yet [!]");
        println!("===================================");
    } else {
        for cust in 0..(customer_data.len()) {
            println!("Customer name: {} ", customer_data[cust].name);
            println!("Customer cart: ");
            for item in &customer_data[cust].cart{
                println!("- {} ", item);
            }
            println!("Total shopping cost: {}", customer_data[cust].total_cost);
            println!("===================================");
        }
    }
}

fn main() {
    println!("=========== WELCOME TO SHOPADA! =============");

    // Initialize main product and customer vector
    let mut products:Vec<Product> = Vec::new();
    let mut customers:Vec<Customer> = Vec::new();

    // Main loop, control of user using their input to the variable.
    loop {
        display_menu();
        let mut choice = String::new(); //init choice var

        println!("Enter choice: ");
        io::stdin().read_line(&mut choice).expect("Error"); //wait for user choice
        let choice:isize = choice.trim().parse().expect("Error");

        if choice == 1 {

            add_product(&mut products);

        }else if choice ==2{

            if products.len() == 0{
                println!("\n[!] PRODUCT LIST IS STILL EMPTY [!]\n");
                println!("=======================================");
            }else {

                let mut name = String::new();
                println!("Enter customer name: ");
                io::stdin().read_line(&mut name).expect("Error");

                add_customer(&mut customers, &mut name);
                edit_customer(&mut customers, &mut products, &mut name)
            }

        }
        else if choice ==3{

            let mut id = String::new();
            println!("Enter product ID to be edited: ");
            io::stdin().read_line(&mut id).expect("Error");
            let id:u32 = id.trim().parse().expect("Error");
            
            edit_product(&mut products, id);

        }
        else if choice ==4{

            let mut id = String::new();
            println!("Enter product ID to be deleted: ");
            io::stdin().read_line(&mut id).expect("Error");
            let id:u32 = id.trim().parse().expect("Error");
            
            delete_product(&mut products, id);

        }
        else if choice ==5 {

            show_products(&mut products);

        } else if choice == 6{

            show_customers(&mut customers);

        }
        else if choice == 7 {

            break;
            
        } else {

            println!("\n [!] Invalid user input [!] \n");

        }
    }
}
