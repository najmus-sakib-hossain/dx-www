package main

import (
    "fmt"
    "os"
)

// Greet function generates a greeting message
func Greet(name string) string {
    return fmt.Sprintf("Hello, %s!", name)
}

// main is the entry point of the program
func main() {
    if len(os.Args) < 2 {
        fmt.Println("Usage: program <name>")
        os.Exit(1)
    }
    
    name := os.Args[1]
    message := Greet(name)
    fmt.Println(message)
}

// BadlyFormattedFunction has intentional formatting issues
func BadlyFormattedFunction(   x   int,y int   ) int{
return x+y
}

// StructExample demonstrates struct definition
type Person struct {
    Name string
    Age  int
    Email string
}

// MethodExample shows method on struct
func (p *Person) String() string {
    return fmt.Sprintf("%s (%d years old)", p.Name, p.Age)
}
