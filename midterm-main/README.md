# CSE262 - Programming Languages - Spring 2023

# Midterm Exam

⏰ **Due by: 4/16/2023 EOD**

## Ethics Contract

**FIRST**: Please read the following carefully:

- I am the sole author of the content within this exam unless otherwise cited.
- I am not an uncredited author of any content in any other exam.
- I will not dicsuss the exam until after the submission deadline.
- All resources I used (including text books and online references, websites, language models), are cited in this exam.
- I will not plagiarize someone else's work and turn it in as my own. If I use someone else's work in this exam, I will cite that work. Failure to cite work I used is plagiarism.
- I understand that acts of academic dishonesty may be penalized to the full extent allowed by the [Lehigh University Code of Conduct][0], including receiving a failing grade for the course. I recognize that I am responsible for understanding the provisions of the Lehigh University Code of Conduct as they relate to this academic exercise.


If you agree with the above, type your full name next to the pen emoji, along with the date. Your exam **will not be graded** without this assent.

---------------------------------------------
🖋️ Zachary Frain 4/16/23
---------------------------------------------

💥 **IMPORTANT:** When you are done, make your first commit with the commit message: `I, <your full name here>, agree to the ethics contract`.

💥 **IMPORTANT: As you are working on your midterm, commit your progress regularly.**

## Exam

Consider the following examples of while loops in different programming langauges:

### Rust

```rust
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}
```

### Python

```python
i = 0
while i < 5:
    print(i)
    i += 1
```

### Java

```java
int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```

### Ruby

```ruby
i = 0
while i < 5 do
    puts i
    i += 1
end
```

### PHP

```php
$i = 0;
while ($i < 5) {
    echo $i;
    $i++;
}
```

## Part 1 - Grammar

Choose one of the above while loops and write a grammar for it in EBNF syntax. In table, first define the symbols you use in your grammar, with the semantic meaning of each.

Whichever language you choose, show other examples of while-loop syntax in that language. Try to cover the edge cases. You may source these examples from the Internet, but be sure to cite them.

## Part 2 - Parser

Write a parser for your grammer in Rust. You may use the nom parser combinator library, but not a parser generator.

In a file called `examples.<extension>` (use the appropriate extension for the language you choose. e.g. for rust do `examples.rs`):

Show 3 examples of while loop code that your parser accepts, along with the parse tree (or AST) it generates.

Show 2 examples of while loop code that your parser rejects, along with the partial parse tree (or AST) it generates (the tree up until the point it fails).

## Part 3 - Code Demo and Explanation

This is the oral portion of the exam. You will record an explanation for your data structure which demonstrates its usage and implementation. You don't have to show your face but you do have to record your voice (accommodations are available upon request). You should be sure to cover the following points in your discussion:

- **Purpose and functionality:** Explain what your code does and how it works.

- **Grammar:** Which language did you choose and why? How did you design your grammar? What edge cases did you consider and how did you build them into the grammar?

- **Parser:** Explain how your code is organized and structured, discuss any design decisions you made. You should also discuss your coding style and any coding conventions you followed. Demonstrate that the parser matches the grammar by showing what it accepts. 

If you didn't finish the exam in is entirety, explain how you attempted to solve it and where you got stuck. This will get you at least some points. 

You can use Zoom to do this, [here is a link](https://support.zoom.us/hc/en-us/articles/360059781332-Getting-started-with-recording) to some instructions. You don't have to record your face, only your voice and the screen. Go through the answer and explain how you arrived there. Your goal with this question is to convince me you know what you are talking about, so I want you to do this without reading a script or written answer. Just go through line by line and explain what the program does. When you are done, upload your recording to your Lehigh Drive and add a link below. 

**💥IMPORTANT: Make sure you give blanket permission to the link holder to view the file**

🎥 Paste Recording Link(s) Here: https://drive.google.com/file/d/1jKhLRMkxShkI7CM-qxaoh3zLScgQM8pq/view?usp=sharing


## Evaluation

- Only files under version control in your forked assignment repository will be graded. Local files left untracked on your computer will not be considered.

- Only code committed *and pushed* prior to the time of grading will be accepted. Locally committed but unpushed code will not be considered.

- Your assignment will be graded according to the [Programming Assignment Grading Rubric](https://drive.google.com/open?id=1V0nBt3Rz6uFMZ9mIaFioLF-48DFX0VdkbgRUDM_eIFk).





