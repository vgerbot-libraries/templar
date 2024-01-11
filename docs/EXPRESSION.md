# Templar 表达式语法文档

表达式语法是 Templar 中非常重要的一部分，它定义了如何进行计算和操作数据。通过合理利用表达式语法，你可以在模板中实现各种动态的逻辑和展示效果。

## 基本表达式

### 变量引用

变量是表达式中的基本构建块。你可以通过 `@{variable}` 的形式引用变量。

**示例:**

```templar
@var{
    name = "John",
    greeting = "Hello"
}
Welcome, @{name}!  // Output: Welcome, John!
```

### 字符串拼接

使用 `+` 运算符可以将字符串进行拼接。

**示例:**

```templar
@var{
    greeting = "Hello",
    name = "World"
}
@{greeting + ", " + name + "!"}  // Output: Hello, World!
```

### 算术运算

支持常见的算术运算符，如 `+`、`-`、`*`、`/`。

**示例:**

```templar
@var{
    num1 = 10,
    num2 = 5
}
@{num1 + num2}  // Output: 15
```

## 条件表达式

### `@if` 语句

使用 `@if` 语句进行条件判断。

**示例:**

```templar
@var{
    age = 20
}
@if(age > 18) {
    Adult
} @else {
    Minor
}
```

### 三元运算符

使用三元运算符进行紧凑的条件判断。

**示例:**

```templar
@var{
    temperature = 25
}
The weather is @{temperature > 20 ? "warm" : "cool"}.
```

## 循环表达式

### `@for` 循环

通过 `@for` 循环遍历数组或对象。

**示例:**

```templar
@var{
    colors = ["red", "green", "blue"]
    person = {name: "Alice", age: 30, city: "Wonderland"}
}
@for(index,color in colors) {
    index:@{index}, Value: @{color}
}

@for(key,value in person) {
    Key: @{key}, Value: @{value}
}
```

## 函数调用

### 内置函数

调用内置函数进行字符串、数学等操作。

**示例:**

```templar
@var{
    text = "Hello, World!"
}
Uppercase: @{toUpperCase(text)}  // Output: HELLO, WORLD!
```

### 用户定义函数

调用自定义函数进行更灵活的操作。

**示例:**

```templar
@function.double(num) {
    @{num * 2}
}

@var{
    number = 5
}
Double of 5 is: @{double(number)}  // Output: 10
```

以上是 Templar 表达式语法的一些基本概念和示例。通过合理利用这些语法，你可以在模板中实现丰富的逻辑和动态内容。如需更详细的信息，请参阅 Templar 的完整文档或帮助资源。
