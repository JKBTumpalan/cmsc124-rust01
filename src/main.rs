use std::io;

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

fn get_unique_id(product_data: &mut Vec<Product>) -> u32{
    loop {
        let mut id = String::new();
        println!("Enter product ID: ");
        io::stdin().read_line(&mut id).expect("Error"); //wait for user input
        let id:u32 = id.trim().parse().expect("Error");

        if check_id(product_data, id) == true{
            return id;
        }
    }
}

fn delete_product(product_data: &mut Vec<Product>, id:u32){
    if product_data.len() == 0{
        println!("Product list is empty, cannot delete anything.");
    }
    for item in 0..(product_data.len()){
        if product_data[item].product_id == id{
            println!("Product {} deleted!", product_data[item].product_id);
            product_data.remove(item);
            break;
        }
        else if (item == (product_data.len()-1)) && (product_data[item].product_id != id){
            println!("Product id not found.");
        }
    }
}

fn edit_product(product_data: &mut Vec<Product>, id:u32){
    if product_data.len() == 0{
        println!("Product list is empty, cannot edit anything.");
    }
    for item in 0..(product_data.len()){
        if product_data[item].product_id == id{

            let mut price = String::new();
            let mut quantity = String::new();
        
            println!("Enter new product price: ");
            io::stdin().read_line(&mut price).expect("Error"); //wait for user input
            let price:f64 = price.trim().parse().expect("Error");
        
            println!("Enter new product quantity: ");
            io::stdin().read_line(&mut quantity).expect("Error"); //wait for user input
            let quantity:u32 = quantity.trim().parse().expect("Error");
        
            product_data[item].price = price;
            product_data[item].quantity = quantity;

            break;
        }
        else if (item == (product_data.len()-1)) && (product_data[item].product_id != id){
            println!("Product id not found.");
        }
    }
}

fn edit_customer(customer_data: &mut Vec<Customer>, product_data: &mut Vec<Product>, customer_name: &mut String){
    if product_data.len() == 0{
        println!("Product list is empty, cannot buy anything.");
    } else {
        show_products(product_data);
    }

    let mut to_buy = String::new();
    println!("Enter product ID to buy: ");
    io::stdin().read_line(&mut to_buy).expect("Error"); //wait for user input
    let to_buy:usize = to_buy.trim().parse().expect("Error");

    if product_data[to_buy-1].quantity > 0{
        for customer in customer_data{
            if customer.name.eq(customer_name){
                customer.cart.push((&product_data[to_buy-1].product_name).to_string());
                customer.total_cost = customer.total_cost + product_data[to_buy-1].price;
                product_data[to_buy-1].quantity = product_data[to_buy-1].quantity - 1;
                break;
            }
        }
    }

}

fn add_product(product_data: &mut Vec<Product>){
    let mut name = String::new();
    let mut price = String::new();
    let mut quantity = String::new();

    let id = get_unique_id(product_data);

    println!("Enter product name: ");
    io::stdin().read_line(&mut name).expect("Error"); //wait for user input

    println!("Enter product price: ");
    io::stdin().read_line(&mut price).expect("Error"); //wait for user input
    let price:f64 = price.trim().parse().expect("Error");

    println!("Enter product quantity: ");
    io::stdin().read_line(&mut quantity).expect("Error"); //wait for user input
    let quantity:u32 = quantity.trim().parse().expect("Error");

    let new_product = Product {
        product_id: id,
        product_name: name,
        price: price,
        quantity: quantity
    };

    println!("=====================================");
    println!("[!] New product added to the shop [!]");
    println!("{} - {} : {}PHP ({} pcs)", &new_product.product_id, &new_product.product_name[0..(&new_product.product_name.len()-1)], &new_product.price, &new_product.quantity);
    println!("=====================================");

    product_data.push(new_product);
}

fn add_customer(customer_data: &mut Vec<Customer>, customer_name: &mut String){
    let mut flag = true;

    let new_customer = Customer {
        name: (&customer_name).to_string(),
        cart: Vec::new(),
        total_cost: 0.0,
    };

    for cust in 0..customer_data.len(){
        if customer_data[cust].name.eq(&new_customer.name){
            println!("Customer is already in the database.");
            flag = false;
            break;
        } else if cust == customer_data.len()-1 && customer_data[cust].name.ne(&new_customer.name){
            println!("=====================================");
            println!("[!] New customer added to the shop [!]");
            println!("{}", new_customer.name);
            println!("=====================================");
        }
    }

    if flag{
        customer_data.push(new_customer);
    }
}

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

fn show_customers(customer_data: &mut Vec<Customer>){
    println!("============ PRODUCTS =============");
    if customer_data.len() == 0{
        println!("[!] There are no customers yet [!]")
    } else {
        for cust in 0..(customer_data.len()) {
            println!("Customer name: {} ", customer_data[cust].name);
            println!("Customer cart: ");
            for item in &customer_data[cust].cart{
                println!("- {} ", item);
            }
            println!("Total shopping cost: {}", customer_data[cust].total_cost)
        }
    }
    println!("===================================");
}

fn main() {
    println!("=========== WELCOME TO SHOPADA! =============");

    let mut products:Vec<Product> = Vec::new();
    let mut customers:Vec<Customer> = Vec::new();

    loop {
        display_menu();
        let mut choice = String::new();

        println!("Enter choice: ");
        io::stdin().read_line(&mut choice).expect("Error"); //wait for user input
        let choice:isize = choice.trim().parse().expect("Error");

        if choice == 1 {
            add_product(&mut products);
        }else if choice ==2{
            let mut name = String::new();
            println!("Enter customer name: ");
            io::stdin().read_line(&mut name).expect("Error"); //wait for user input

            add_customer(&mut customers, &mut name);
            edit_customer(&mut customers, &mut products, &mut name)
        }
        else if choice ==3{
            let mut id = String::new();
            println!("Enter product ID to be edited: ");
            io::stdin().read_line(&mut id).expect("Error"); //wait for user input
            let id:u32 = id.trim().parse().expect("Error");
            
            edit_product(&mut products, id);
        }
        else if choice ==4{
            let mut id = String::new();
            println!("Enter product ID to be deleted: ");
            io::stdin().read_line(&mut id).expect("Error"); //wait for user input
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
        }
    }
}
