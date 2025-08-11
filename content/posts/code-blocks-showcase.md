---
title: "Code Blocks Showcase"
date: 2024-01-16T12:00:00Z
tags: ["demo", "code", "languages"]
excerpt: "A comprehensive showcase of all supported programming languages with executable examples"
---

# Code Blocks Showcase

This post demonstrates all supported programming languages with syntax highlighting and execution capabilities.

## Web Languages

### JavaScript
```javascript
// JavaScript with console output
console.log("Hello from JavaScript!");
console.log("Line 2 of output");

const add = (a, b) => a + b;
console.log("2 + 3 =", add(2, 3));

// Loop printing numbers
for (let i = 1; i <= 5; i++) {
    console.log(`Number: ${i}`);
}

// Return value
add(10, 20);
```

### TypeScript
```typescript
// TypeScript example
interface User {
    name: string;
    age: number;
}

const user: User = {
    name: "John",
    age: 30
};

console.log(user);

// Loop printing numbers
for (let i = 1; i <= 5; i++) {
    console.log(`Number: ${i}`);
}
```

### HTML
```html
<!DOCTYPE html>
<html>
<head>
    <title>Test Page</title>
</head>
<body>
    <h1>Hello HTML!</h1>
    <p>This is a test paragraph.</p>
</body>
</html>
```

### CSS
```css
/* CSS styling example */
body {
    margin: 0;
    padding: 20px;
    font-family: system-ui, sans-serif;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
}
```

## Systems Languages

### Rust
```rust
fn main() {
    println!("Hello from Rust!");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    println!("Sum: {}", sum);
    println!("Count: {}", numbers.len());
    
    // Loop printing numbers
    for i in 1..=5 {
        println!("Number: {}", i);
    }
}
```

### C
```c
#include <stdio.h>

int main() {
    printf("Hello from C!\n");
    
    int arr[] = {1, 2, 3, 4, 5};
    int sum = 0;
    
    for(int i = 0; i < 5; i++) {
        sum += arr[i];
    }
    
    printf("Sum: %d\n", sum);
    
    // Loop printing numbers
    for(int i = 1; i <= 5; i++) {
        printf("Number: %d\n", i);
    }
    
    return 0;
}
```

### C++
```cpp
#include <iostream>
#include <vector>

int main() {
    std::cout << "Hello from C++!" << std::endl;
    
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    int sum = 0;
    
    for(int n : numbers) {
        sum += n;
    }
    
    std::cout << "Sum: " << sum << std::endl;
    
    // Loop printing numbers
    for(int i = 1; i <= 5; i++) {
        std::cout << "Number: " << i << std::endl;
    }
    
    return 0;
}
```

### Go
```go
package main

import "fmt"

func main() {
    fmt.Println("Hello from Go!")
    
    numbers := []int{1, 2, 3, 4, 5}
    sum := 0
    
    for _, n := range numbers {
        sum += n
    }
    
    fmt.Printf("Sum: %d\n", sum)
    
    // Loop printing numbers
    for i := 1; i <= 5; i++ {
        fmt.Printf("Number: %d\n", i)
    }
}
```

### Zig
```zig
const std = @import("std");

pub fn main() void {
    std.debug.print("Hello from Zig!\n", .{});
    
    const numbers = [_]i32{ 1, 2, 3, 4, 5 };
    var sum: i32 = 0;
    
    for (numbers) |n| {
        sum += n;
    }
    
    std.debug.print("Sum: {}\n", .{sum});
    
    // Loop printing numbers
    var i: i32 = 1;
    while (i <= 5) : (i += 1) {
        std.debug.print("Number: {}\n", .{i});
    }
}
```

## Scripting Languages

### Python
```python
# Python example
print("Hello from Python!")

numbers = [1, 2, 3, 4, 5]
total = sum(numbers)

print(f"Sum: {total}")
print(f"Average: {total / len(numbers)}")

# Loop printing numbers
for i in range(1, 6):
    print(f"Number: {i}")

# List comprehension
squares = [x**2 for x in numbers]
print(f"Squares: {squares}")
```

### Ruby
```ruby
# Ruby example
puts "Hello from Ruby!"

numbers = [1, 2, 3, 4, 5]
sum = numbers.sum

puts "Sum: #{sum}"
puts "Average: #{sum.to_f / numbers.length}"

# Loop printing numbers
(1..5).each do |i|
  puts "Number: #{i}"
end

# Map operation
squares = numbers.map { |x| x ** 2 }
puts "Squares: #{squares}"
```

### PHP
```php
<?php
// PHP example
echo "Hello from PHP!\n";

$numbers = [1, 2, 3, 4, 5];
$sum = array_sum($numbers);

echo "Sum: $sum\n";
echo "Average: " . ($sum / count($numbers)) . "\n";

// Loop printing numbers
for ($i = 1; $i <= 5; $i++) {
    echo "Number: $i\n";
}

// Array map
$squares = array_map(function($x) { return $x ** 2; }, $numbers);
echo "Squares: " . implode(", ", $squares) . "\n";
?>
```

### Perl
```perl
#!/usr/bin/perl
# Perl example
print "Hello from Perl!\n";

my @numbers = (1, 2, 3, 4, 5);
my $sum = 0;
$sum += $_ for @numbers;

print "Sum: $sum\n";
print "Average: " . ($sum / @numbers) . "\n";

# Loop printing numbers
for my $i (1..5) {
    print "Number: $i\n";
}

# Map operation
my @squares = map { $_ ** 2 } @numbers;
print "Squares: @squares\n";
```

### Lua
```lua
-- Lua example
print("Hello from Lua!")

numbers = {1, 2, 3, 4, 5}
sum = 0

for i, v in ipairs(numbers) do
    sum = sum + v
end

print("Sum: " .. sum)
print("Average: " .. sum / #numbers)

-- Loop printing numbers
for i = 1, 5 do
    print("Number: " .. i)
end
```

## Functional Languages

### Haskell
```haskell
-- Haskell example
main = do
    putStrLn "Hello from Haskell!"
    
    let numbers = [1, 2, 3, 4, 5]
    let total = sum numbers
    
    putStrLn $ "Sum: " ++ show total
    putStrLn $ "Average: " ++ show (fromIntegral total / fromIntegral (length numbers))
    
    -- Loop printing numbers
    mapM_ (\i -> putStrLn $ "Number: " ++ show i) [1..5]
    
    let squares = map (^2) numbers
    putStrLn $ "Squares: " ++ show squares
```

### Elixir
```elixir
# Elixir example
IO.puts "Hello from Elixir!"

numbers = [1, 2, 3, 4, 5]
sum = Enum.sum(numbers)

IO.puts "Sum: #{sum}"
IO.puts "Average: #{sum / length(numbers)}"

# Loop printing numbers
Enum.each(1..5, fn i -> IO.puts "Number: #{i}" end)

squares = Enum.map(numbers, fn x -> x * x end)
IO.puts "Squares: #{inspect squares}"
```

### F#
```fsharp
// F# example
printfn "Hello from F#!"

let numbers = [1; 2; 3; 4; 5]
let sum = List.sum numbers

printfn "Sum: %d" sum
printfn "Average: %f" (float sum / float numbers.Length)

// Loop printing numbers
[1..5] |> List.iter (fun i -> printfn "Number: %d" i)

let squares = List.map (fun x -> x * x) numbers
printfn "Squares: %A" squares
```

## JVM Languages

### Java
```java
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello from Java!");
        
        int[] numbers = {1, 2, 3, 4, 5};
        int sum = 0;
        
        for (int n : numbers) {
            sum += n;
        }
        
        System.out.println("Sum: " + sum);
        System.out.println("Average: " + (double)sum / numbers.length);
        
        // Loop printing numbers
        for (int i = 1; i <= 5; i++) {
            System.out.println("Number: " + i);
        }
    }
}
```

### Kotlin
```kotlin
fun main() {
    println("Hello from Kotlin!")
    
    val numbers = listOf(1, 2, 3, 4, 5)
    val sum = numbers.sum()
    
    println("Sum: $sum")
    println("Average: ${sum.toDouble() / numbers.size}")
    
    // Loop printing numbers
    for (i in 1..5) {
        println("Number: $i")
    }
    
    val squares = numbers.map { it * it }
    println("Squares: $squares")
}
```

### Scala
```scala
object Main extends App {
    println("Hello from Scala!")
    
    val numbers = List(1, 2, 3, 4, 5)
    val sum = numbers.sum
    
    println(s"Sum: $sum")
    println(s"Average: ${sum.toDouble / numbers.length}")
    
    // Loop printing numbers
    (1 to 5).foreach(i => println(s"Number: $i"))
    
    val squares = numbers.map(x => x * x)
    println(s"Squares: $squares")
}
```

## Shell Languages

### Bash
```bash
#!/bin/bash
echo "Hello from Bash!"

numbers=(1 2 3 4 5)
sum=0

for n in "${numbers[@]}"; do
    sum=$((sum + n))
done

echo "Sum: $sum"
echo "Count: ${#numbers[@]}"

# Loop printing numbers
for i in {1..5}; do
    echo "Number: $i"
done
```

### PowerShell
```powershell
Write-Host "Hello from PowerShell!"

$numbers = 1, 2, 3, 4, 5
$sum = ($numbers | Measure-Object -Sum).Sum

Write-Host "Sum: $sum"
Write-Host "Average: $($sum / $numbers.Count)"

# Loop printing numbers
for ($i = 1; $i -le 5; $i++) {
    Write-Host "Number: $i"
}

$squares = $numbers | ForEach-Object { $_ * $_ }
Write-Host "Squares: $squares"
```

## Data/Configuration Languages

### JSON
```json
{
  "name": "Code Blocks Demo",
  "version": "1.0.0",
  "languages": ["JavaScript", "Python", "Rust", "Go"],
  "features": {
    "syntax_highlighting": true,
    "execution": true,
    "copy_button": true
  }
}
```

### YAML
```yaml
name: Code Blocks Demo
version: 1.0.0
languages:
  - JavaScript
  - Python
  - Rust
  - Go
features:
  syntax_highlighting: true
  execution: true
  copy_button: true
```

### TOML
```toml
[package]
name = "code-blocks-demo"
version = "1.0.0"

[features]
syntax_highlighting = true
execution = true
copy_button = true

languages = ["JavaScript", "Python", "Rust", "Go"]
```

### XML
```xml
<?xml version="1.0" encoding="UTF-8"?>
<project>
    <name>Code Blocks Demo</name>
    <version>1.0.0</version>
    <languages>
        <language>JavaScript</language>
        <language>Python</language>
        <language>Rust</language>
        <language>Go</language>
    </languages>
</project>
```

## SQL
```sql
-- SQL example
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100),
    age INTEGER
);

INSERT INTO users (name, age) VALUES 
    ('Alice', 25),
    ('Bob', 30),
    ('Charlie', 35);

SELECT * FROM users WHERE age > 25;
```

## Regular Expressions
```regex
# Email validation regex
^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$

# Phone number (US format)
^\(?([0-9]{3})\)?[-. ]?([0-9]{3})[-. ]?([0-9]{4})$
```

## Conclusion

This showcase demonstrates the wide variety of programming languages supported by our code block system. Each language has proper syntax highlighting, and many support direct execution in the browser!