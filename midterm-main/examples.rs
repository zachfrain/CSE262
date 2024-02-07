//Since I wasn't able to complete my program, this is what I envision should happen if my ideas were in the right direction

//Success One
/*
int i = 0;
while(i < 1) {
    System.out.println("h")
    i++;
}
Parsed Tree:

Program { 
    children: [
            VariableDefine { 
                children: [
                    Type {
                        value: "int" 
                    }, 
                    Identifier { 
                        value: "i" 
                    }, Expression { 
                        children: [
                            Number { 
                                value: 0 
                            }] 
                    }
                ] 
            }, 
            WhileLoop { 
                children: [
                    Condition { 
                        value: -1, 
                        children: [
                            Expression {
                                children: [
                                    Identifier { 
                                        value: "i" 
                                    }
                                    ] 
                            }, Expression { 
                                children: [
                                    Number { 
                                        value: 1 
                                    }
                                    ] 
                    }] }, 
                    PrintStatement { 
                        children: [
                            String { 
                                value: "h"
                            }
                        ] 
                    }, SingleMathExpression { 
                        name: "+", 
                        children: [
                            Identifier { 
                                value: "i"
                            }, Number { 
                                value: 1 
                            }
                        ] 
                    }] 
                }
            ] 
        }
 */

//Success Two
/*
int i = 2;
while(i > 1){
    System.out.print("h");
    i--;
}
Parsed Tree:

Program { 
    children: [
            VariableDefine { 
                children: [
                    Type {
                        value: "int" 
                    }, 
                    Identifier { 
                        value: "i" 
                    }, Expression { 
                        children: [
                            Number { 
                                value: 2 
                            }] 
                    }
                ] 
            }, 
            WhileLoop { 
                children: [
                    Condition { 
                        value: 1, 
                        children: [
                            Expression {
                                children: [
                                    Identifier { 
                                        value: "i" 
                                    }
                                    ] 
                            }, Expression { 
                                children: [
                                    Number { 
                                        value: 1 
                                    }
                                    ] 
                    }] }, 
                    PrintStatement { 
                        children: [
                            String { 
                                value: "h"
                            }
                        ] 
                    }, SingleMathExpression { 
                        name: "-", 
                        children: [
                            Identifier { 
                                value: "i"
                            }, Number { 
                                value: 1 
                            }
                        ] 
                    }] 
                }
            ] 
        }


 */

//Success Three
/*
int i = 0;
while(i == 0){
    System.out.println("h");
    i++;
}
Parsed Tree:

Program { 
    children: [
            VariableDefine { 
                children: [
                    Type {
                        value: "int" 
                    }, 
                    Identifier { 
                        value: "i" 
                    }, Expression { 
                        children: [
                            Number { 
                                value: 0 
                            }] 
                    }
                ] 
            }, 
            WhileLoop { 
                children: [
                    Condition { 
                        value: 0, 
                        children: [
                            Expression {
                                children: [
                                    Identifier { 
                                        value: "i" 
                                    }
                                    ] 
                            }, Expression { 
                                children: [
                                    Number { 
                                        value: 0 
                                    }
                                    ] 
                    }] }, 
                    PrintStatement { 
                        children: [
                            String { 
                                value: "h"
                            }
                        ] 
                    }, SingleMathExpression { 
                        name: "+", 
                        children: [
                            Identifier { 
                                value: "i"
                            }, Number { 
                                value: 1 
                            }
                        ] 
                    }] 
                }
            ] 
        }
 */

//Reject One
/*
int i = 1;
while(i < 20){
    System.out.println("h");
    i*=10;
}
Parsed Tree:

Program { 
    children: [
            VariableDefine { 
                children: [
                    Type {
                        value: "int" 
                    }, 
                    Identifier { 
                        value: "i" 
                    }, Expression { 
                        children: [
                            Number { 
                                value: 1 
                            }] 
                    }
            ] 
}

Unparsed Input: "while(i<20) { System.out.println(\ "h\"); i*=10;"


 */

//Reject Two
/*
int i = 15;
while(i > 20){
    System.out.println("h");
    i/=3;
}

Parsed Tree:
Program { 
    children: [
            VariableDefine { 
                children: [
                    Type {
                        value: "int" 
                    }, 
                    Identifier { 
                        value: "i" 
                    }, Expression { 
                        children: [
                            Number { 
                                value: 15 
                            }] 
                    }
            ] 
    }

Unparsed Input: "while(i>20){ System.out.println(\"h\"); i/=3;}"

 */