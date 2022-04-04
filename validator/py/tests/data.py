# SPDX-License-Identifier: Apache-2.0

BASIC_PLAN = """
{
 "extensionUris": [],
 "extensions": [],
 "relations": [
  {
   "rel": {
    "project": {
     "input": {
      "read": {
       "common": {
        "direct": {}
       },
       "projection": {
        "select": {
         "structItems": [
          {
           "field": 0
          },
          {
           "field": 1
          }
         ]
        },
        "maintainSingularStruct": false
       },
       "namedTable": {
        "names": [
         "person"
        ]
       }
      }
     },
     "expressions": [
      {
       "selection": {
        "directReference": {
         "structField": {
          "field": 0
         }
        }
       }
      },
      {
       "selection": {
        "directReference": {
         "structField": {
          "field": 1
         }
        }
       }
      }
     ]
    }
   }
  }
 ],
 "expectedTypeUrls": []
}
"""

COMPLEX_PLAN = """
{
  "extensionUris": [{
    "extensionUriAnchor": 1,
    "uri": "/functions_boolean.yaml"
  }, {
    "extensionUriAnchor": 4,
    "uri": "/functions_arithmetic_decimal.yaml"
  }, {
    "extensionUriAnchor": 3,
    "uri": "/functions_datetime.yaml"
  }, {
    "extensionUriAnchor": 2,
    "uri": "/functions_comparison.yaml"
  }],
  "extensions": [{
    "extensionFunction": {
      "extensionUriReference": 1,
      "functionAnchor": 1,
      "name": "and:bool"
    }
  }, {
    "extensionFunction": {
      "extensionUriReference": 2,
      "functionAnchor": 2,
      "name": "equal:any1_any1"
    }
  }, {
    "extensionFunction": {
      "extensionUriReference": 3,
      "functionAnchor": 3,
      "name": "lt:date_date"
    }
  }, {
    "extensionFunction": {
      "extensionUriReference": 3,
      "functionAnchor": 4,
      "name": "gt:date_date"
    }
  }, {
    "extensionFunction": {
      "extensionUriReference": 4,
      "functionAnchor": 5,
      "name": "multiply:opt_decimal_decimal"
    }
  }, {
    "extensionFunction": {
      "extensionUriReference": 4,
      "functionAnchor": 6,
      "name": "subtract:opt_decimal_decimal"
    }
  }, {
    "extensionFunction": {
      "extensionUriReference": 4,
      "functionAnchor": 7,
      "name": "sum:opt_decimal"
    }
  }],
  "relations": [{
    "root": {
      "input": {
        "fetch": {
          "common": {
            "direct": {
            }
          },
          "input": {
            "sort": {
              "common": {
                "direct": {
                }
              },
              "input": {
                "project": {
                  "common": {
                    "emit": {
                      "outputMapping": [4, 5, 6, 7]
                    }
                  },
                  "input": {
                    "aggregate": {
                      "common": {
                        "direct": {
                        }
                      },
                      "input": {
                        "project": {
                          "common": {
                            "emit": {
                              "outputMapping": [33, 34, 35, 36]
                            }
                          },
                          "input": {
                            "filter": {
                              "common": {
                                "direct": {
                                }
                              },
                              "input": {
                                "join": {
                                  "common": {
                                    "direct": {
                                    }
                                  },
                                  "left": {
                                    "join": {
                                      "common": {
                                        "direct": {
                                        }
                                      },
                                      "left": {
                                        "read": {
                                          "common": {
                                            "direct": {
                                            }
                                          },
                                          "baseSchema": {
                                            "names": ["C_CUSTKEY", "C_NAME", "C_ADDRESS", "C_NATIONKEY", "C_PHONE", "C_ACCTBAL", "C_MKTSEGMENT", "C_COMMENT"],
                                            "struct": {
                                              "types": [{
                                                "i64": {
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_REQUIRED"
                                                }
                                              }, {
                                                "varchar": {
                                                  "length": 25,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "varchar": {
                                                  "length": 40,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "i64": {
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_REQUIRED"
                                                }
                                              }, {
                                                "fixedChar": {
                                                  "length": 15,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "decimal": {
                                                  "scale": 0,
                                                  "precision": 19,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "fixedChar": {
                                                  "length": 10,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "varchar": {
                                                  "length": 117,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }],
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_REQUIRED"
                                            }
                                          },
                                          "namedTable": {
                                            "names": ["CUSTOMER"]
                                          }
                                        }
                                      },
                                      "right": {
                                        "read": {
                                          "common": {
                                            "direct": {
                                            }
                                          },
                                          "baseSchema": {
                                            "names": ["O_ORDERKEY", "O_CUSTKEY", "O_ORDERSTATUS", "O_TOTALPRICE", "O_ORDERDATE", "O_ORDERPRIORITY", "O_CLERK", "O_SHIPPRIORITY", "O_COMMENT"],
                                            "struct": {
                                              "types": [{
                                                "i64": {
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_REQUIRED"
                                                }
                                              }, {
                                                "i64": {
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_REQUIRED"
                                                }
                                              }, {
                                                "fixedChar": {
                                                  "length": 1,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "decimal": {
                                                  "scale": 0,
                                                  "precision": 19,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "date": {
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "fixedChar": {
                                                  "length": 15,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "fixedChar": {
                                                  "length": 15,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "i32": {
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }, {
                                                "varchar": {
                                                  "length": 79,
                                                  "typeVariationReference": 0,
                                                  "nullability": "NULLABILITY_NULLABLE"
                                                }
                                              }],
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_REQUIRED"
                                            }
                                          },
                                          "namedTable": {
                                            "names": ["ORDERS"]
                                          }
                                        }
                                      },
                                      "expression": {
                                        "literal": {
                                          "boolean": true,
                                          "nullable": false
                                        }
                                      },
                                      "type": "JOIN_TYPE_INNER"
                                    }
                                  },
                                  "right": {
                                    "read": {
                                      "common": {
                                        "direct": {
                                        }
                                      },
                                      "baseSchema": {
                                        "names": ["L_ORDERKEY", "L_PARTKEY", "L_SUPPKEY", "L_LINENUMBER", "L_QUANTITY", "L_EXTENDEDPRICE", "L_DISCOUNT", "L_TAX", "L_RETURNFLAG", "L_LINESTATUS", "L_SHIPDATE", "L_COMMITDATE", "L_RECEIPTDATE", "L_SHIPINSTRUCT", "L_SHIPMODE", "L_COMMENT"],
                                        "struct": {
                                          "types": [{
                                            "i64": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_REQUIRED"
                                            }
                                          }, {
                                            "i64": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_REQUIRED"
                                            }
                                          }, {
                                            "i64": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_REQUIRED"
                                            }
                                          }, {
                                            "i32": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "decimal": {
                                              "scale": 0,
                                              "precision": 19,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "decimal": {
                                              "scale": 0,
                                              "precision": 19,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "decimal": {
                                              "scale": 0,
                                              "precision": 19,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "decimal": {
                                              "scale": 0,
                                              "precision": 19,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "fixedChar": {
                                              "length": 1,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "fixedChar": {
                                              "length": 1,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "date": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "date": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "date": {
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "fixedChar": {
                                              "length": 25,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "fixedChar": {
                                              "length": 10,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }, {
                                            "varchar": {
                                              "length": 44,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_NULLABLE"
                                            }
                                          }],
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_REQUIRED"
                                        }
                                      },
                                      "namedTable": {
                                        "names": ["LINEITEM"]
                                      }
                                    }
                                  },
                                  "expression": {
                                    "literal": {
                                      "boolean": true,
                                      "nullable": false
                                    }
                                  },
                                  "type": "JOIN_TYPE_INNER"
                                }
                              },
                              "condition": {
                                "scalarFunction": {
                                  "functionReference": 1,
                                  "args": [{
                                    "scalarFunction": {
                                      "functionReference": 2,
                                      "args": [{
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 6
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }, {
                                        "cast": {
                                          "type": {
                                            "fixedChar": {
                                              "length": 10,
                                              "typeVariationReference": 0,
                                              "nullability": "NULLABILITY_REQUIRED"
                                            }
                                          },
                                          "input": {
                                            "literal": {
                                              "fixedChar": "HOUSEHOLD",
                                              "nullable": false
                                            }
                                          }
                                        }
                                      }],
                                      "outputType": {
                                        "bool": {
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_NULLABLE"
                                        }
                                      }
                                    }
                                  }, {
                                    "scalarFunction": {
                                      "functionReference": 2,
                                      "args": [{
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 0
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }, {
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 9
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }],
                                      "outputType": {
                                        "bool": {
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_REQUIRED"
                                        }
                                      }
                                    }
                                  }, {
                                    "scalarFunction": {
                                      "functionReference": 2,
                                      "args": [{
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 17
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }, {
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 8
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }],
                                      "outputType": {
                                        "bool": {
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_REQUIRED"
                                        }
                                      }
                                    }
                                  }, {
                                    "scalarFunction": {
                                      "functionReference": 3,
                                      "args": [{
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 12
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }, {
                                        "literal": {
                                          "date": 9214,
                                          "nullable": false
                                        }
                                      }],
                                      "outputType": {
                                        "bool": {
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_NULLABLE"
                                        }
                                      }
                                    }
                                  }, {
                                    "scalarFunction": {
                                      "functionReference": 4,
                                      "args": [{
                                        "selection": {
                                          "directReference": {
                                            "structField": {
                                              "field": 27
                                            }
                                          },
                                          "rootReference": {
                                          }
                                        }
                                      }, {
                                        "literal": {
                                          "date": 9214,
                                          "nullable": false
                                        }
                                      }],
                                      "outputType": {
                                        "bool": {
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_NULLABLE"
                                        }
                                      }
                                    }
                                  }],
                                  "outputType": {
                                    "bool": {
                                      "typeVariationReference": 0,
                                      "nullability": "NULLABILITY_NULLABLE"
                                    }
                                  }
                                }
                              }
                            }
                          },
                          "expressions": [{
                            "selection": {
                              "directReference": {
                                "structField": {
                                  "field": 17
                                }
                              },
                              "rootReference": {
                              }
                            }
                          }, {
                            "selection": {
                              "directReference": {
                                "structField": {
                                  "field": 12
                                }
                              },
                              "rootReference": {
                              }
                            }
                          }, {
                            "selection": {
                              "directReference": {
                                "structField": {
                                  "field": 15
                                }
                              },
                              "rootReference": {
                              }
                            }
                          }, {
                            "scalarFunction": {
                              "functionReference": 5,
                              "args": [{
                                "selection": {
                                  "directReference": {
                                    "structField": {
                                      "field": 22
                                    }
                                  },
                                  "rootReference": {
                                  }
                                }
                              }, {
                                "scalarFunction": {
                                  "functionReference": 6,
                                  "args": [{
                                    "cast": {
                                      "type": {
                                        "decimal": {
                                          "scale": 0,
                                          "precision": 19,
                                          "typeVariationReference": 0,
                                          "nullability": "NULLABILITY_NULLABLE"
                                        }
                                      },
                                      "input": {
                                        "literal": {
                                          "i32": 1,
                                          "nullable": false
                                        }
                                      }
                                    }
                                  }, {
                                    "selection": {
                                      "directReference": {
                                        "structField": {
                                          "field": 23
                                        }
                                      },
                                      "rootReference": {
                                      }
                                    }
                                  }],
                                  "outputType": {
                                    "decimal": {
                                      "scale": 0,
                                      "precision": 19,
                                      "typeVariationReference": 0,
                                      "nullability": "NULLABILITY_NULLABLE"
                                    }
                                  }
                                }
                              }],
                              "outputType": {
                                "decimal": {
                                  "scale": 0,
                                  "precision": 19,
                                  "typeVariationReference": 0,
                                  "nullability": "NULLABILITY_NULLABLE"
                                }
                              }
                            }
                          }]
                        }
                      },
                      "groupings": [{
                        "groupingExpressions": [{
                          "selection": {
                            "directReference": {
                              "structField": {
                                "field": 0
                              }
                            },
                            "rootReference": {
                            }
                          }
                        }, {
                          "selection": {
                            "directReference": {
                              "structField": {
                                "field": 1
                              }
                            },
                            "rootReference": {
                            }
                          }
                        }, {
                          "selection": {
                            "directReference": {
                              "structField": {
                                "field": 2
                              }
                            },
                            "rootReference": {
                            }
                          }
                        }]
                      }, {
                        "groupingExpressions": [{
                          "selection": {
                            "directReference": {
                              "structField": {
                                "field": 0
                              }
                            },
                            "rootReference": {
                            }
                          }
                        }, {
                          "selection": {
                            "directReference": {
                              "structField": {
                                "field": 1
                              }
                            },
                            "rootReference": {
                            }
                          }
                        }, {
                          "selection": {
                            "directReference": {
                              "structField": {
                                "field": 2
                              }
                            },
                            "rootReference": {
                            }
                          }
                        }]
                      }],
                      "measures": [{
                        "measure": {
                          "functionReference": 7,
                          "args": [{
                            "selection": {
                              "directReference": {
                                "structField": {
                                  "field": 3
                                }
                              },
                              "rootReference": {
                              }
                            }
                          }],
                          "sorts": [],
                          "phase": "AGGREGATION_PHASE_INITIAL_TO_RESULT",
                          "outputType": {
                            "decimal": {
                              "scale": 0,
                              "precision": 19,
                              "typeVariationReference": 0,
                              "nullability": "NULLABILITY_NULLABLE"
                            }
                          }
                        }
                      }]
                    }
                  },
                  "expressions": [{
                    "selection": {
                      "directReference": {
                        "structField": {
                          "field": 0
                        }
                      },
                      "rootReference": {
                      }
                    }
                  }, {
                    "selection": {
                      "directReference": {
                        "structField": {
                          "field": 3
                        }
                      },
                      "rootReference": {
                      }
                    }
                  }, {
                    "selection": {
                      "directReference": {
                        "structField": {
                          "field": 1
                        }
                      },
                      "rootReference": {
                      }
                    }
                  }, {
                    "selection": {
                      "directReference": {
                        "structField": {
                          "field": 2
                        }
                      },
                      "rootReference": {
                      }
                    }
                  }]
                }
              },
              "sorts": [{
                "expr": {
                  "selection": {
                    "directReference": {
                      "structField": {
                        "field": 1
                      }
                    },
                    "rootReference": {
                    }
                  }
                },
                "direction": "SORT_DIRECTION_DESC_NULLS_FIRST"
              }, {
                "expr": {
                  "selection": {
                    "directReference": {
                      "structField": {
                        "field": 2
                      }
                    },
                    "rootReference": {
                    }
                  }
                },
                "direction": "SORT_DIRECTION_ASC_NULLS_LAST"
              }]
            }
          },
          "offset": "0",
          "count": "10"
        }
      },
      "names": ["L_ORDERKEY", "REVENUE", "O_ORDERDATE", "O_SHIPPRIORITY"]
    }
  }],
  "expectedTypeUrls": []
}
"""

BASIC_YAML = """---
types:
  - name: point
    structure:
      latitude: i32
      longitude: i32
  - name: line
    structure:
      start: point
      end: point
"""
