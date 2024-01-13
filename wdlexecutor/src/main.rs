


/*
     * Process Flow
     * Parser gets .wdl file
     * 
     * Emit Tokens... WorkflowToken ->[TaskToken(s)->[InputToken, OutputToken] ]
     * 
     * Things to note in WDL grammar <workflow, task, function, command, call>
     * 
     * Runner/mod -> takes Tokens -> Just Run
     * 
     * Greeter says we're done bye, please come again
     * 
     */
fn main() {
    
    println!("Hello, world!");
    let my_name = "Faith";
    println!("My name is {}", my_name);

    
}
