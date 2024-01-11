# Templar 语法

## 指令

指令是在模板中嵌入逻辑的一种语法。在 Templar 中，指令的基本形式为 `@rule`。

### 指令的基本语法

指令的语法有两种基本形式：

```templar
@instruction(expression) {
    ....
}
```

或者：

```templar
@instruction {
    ....
}
```

以下是以 `@if`/`@else` 指令为例的语法示例：

```templar
@if(name is none) {
    is None
} @else {
    is Not None
}
```

### 指令列表

#### `@if`/`@else`/`@elif`

条件判断

```templar
@if(condition) {
    // code block if condition is true
} @elif(anotherCondition) {
    // code block if anotherCondition is true
} @else {
    // code block if none of the conditions are true
}
```

#### `@case`/`@when`

模式匹配

```templar
@case(value) {
    @when(pattern1) {
        // code block if value matches pattern1
    }
    @when(pattern2) {
        // code block if value matches pattern2
    }
    // ...
    @else {
        // code block if none of the patterns match
    }
}
```

#### `@for`

循环

```templar
@for(value,keyOrIndex in dictOrArray) {
    // code block to be repeated for each keyOrIndex, value pair
}
```

#### `@var`

定义变量，变量可以在后续的表达式中使用

```templar
@var {
    name = expression
}
```

#### `@function`

定义函数

```templar
@function.sum(a, b) {
    @{a + b}
}
```

#### `@include`

包含其他模板文件，但不会引入其变量

```templar
@include("path/to/other/template")
```

#### `@import(file-path){var1, var2}`

从其他模板文件中导入变量，其他模板的内容不会被包含进来。`var1, var2` 可以替换为 `*` 以导入所有变量。

```templar
@import("path/to/other/template"){var1, var2}
```

## 插值语法

插值语法用于在模板中嵌入数据。插值语法为 `@{expression}`。

以下是一个简单的示例：

```templar
1 + 1 = @{1 + 1}
```

这是一个 `@for` 循环示例：

```templar
@var {
    dict = {
        name: "Mario",
        age: 20
    }
}
@for(key, value in dict) {
    @{key}: @{value}
}
```

输出：

```txt
name: Mario
age: 20
```

## 注释

```templar
// 这是单行注释
/*
这是多行注释
*/
```
