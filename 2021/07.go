package main

import "fmt"

func sayHello() {
  fmt.Println("hello");
}

func getHello(name string) string {
  msg := "hello! " + name
  return msg
}

func main() {
  var a int
  var b int
  var c [5]int
  fmt.Printf("a %d\n", a)
  fmt.Println(&a, &b)
  fmt.Println(&c[0], &c[1], &c[2])
  fmt.Println(c[1:3])
  sayHello()
  fmt.Println(getHello("Tom"))
}
