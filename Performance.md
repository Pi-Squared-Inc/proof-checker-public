# CSL Examples

## Risc 0
|     Examples     |  Cycles | CPU Time | GPU Time |
|:----------------:|:-------:|:--------:|:--------:|
| perceptron       |  21156  |   2.359  |   0.622  |
| svm5             |  21156  |   2.641  |   0.621  |
| transfer5000     | 724258  |  37.879  |   7.586  |
| transfer         |  21156  |   2.333  |   0.597  |
		

## zkLLVM
|     Examples     | CPU Circuit Gen Time | CPU Prove+Verify Time | GPU Time |
|:----------------:|:--------------------:|:---------------------:|:--------:|
| perceptron       |                 0.95 |                 0.135 |          |
| svm5             |                 0.96 |                 0.135 |          |
| transfer         |                 0.97 |                 0.133 |          |
| transfer-batch   |                0.797 |                 40.34 |          |


## Lurk
|     Examples     |  Cycles | CPU Time** | GPU Time |
|:----------------:|:-------:|:--------:|:--------:|
| perceptron       |    11   |   3.979  |   2.328  |
| svm5             |    9    |   2.263  |   2.278  |
| transfer5000*    |  330387 | 1766.207 | 481.500  |
| transfer         |    34   |   2.522  |   2.441  |


* Using `lurk --rc 400 transfer5000.lurk`, other tests doesn't use `--rc`

# Proof Checker

## Risc 0
|             Examples            |  Cycles | CPU Time | GPU Time |
|:-------------------------------:|:-------:|:--------:|:--------:|
| perceptron-goal                 | 3196173 |  122.839 |  28.088  |
| svm5-goal                       | 3196173 |  123.670 |  27.998  |
| transfer-batch-1k-goal          | 6724225 |  273.092 |  60.219  |
| transfer-simple-compressed-goal | 1123933 |   48.555 |  10.742  |
| transfer-task-specific          |   89321 |    4.804 |   1.177  |
| impreflex-compressed-goal       |   68555 |    4.740 |   1.156  |

## zkLLVM
|             Examples            |CPU Circuit Gen Time | CPU Prove+Verify Time | GPU Time |
|:-------------------------------:|:-------------------:|:---------------------:|:--------:|
| impreflex-compressed-goal       |               5.798 |                372.76 |          |
| perceptron-goal                 |             359.743 |                     ∞ |          |
| svm5-goal                       |             359.371 |                     ∞ |          |
| transfer-task-specific          |              11.678 |                784.11 |          |
| transfer-simple-compressed-goal |              91.160 |              7188.792 |          |
| transfer-batch-1k-goal          |                ∞    |                     ∞ |          |

## Lurk
|             Examples            | Cycles | CPU Time** | GPU Time |
|:-------------------------------:|:------:|:--------:|:--------:|
| perceptron-goal                 | 6404208|     ∞    |          |
| svm5-goal                       | 6404208|     ∞    |          |
| transfer-batch-1k-goal          |30122047|     ∞    |          |
| transfer-simple-compressed-goal | 3202986|     ∞    |          |
| transfer-task-specific*         | 148870 |  861.443 |  237.319 |
| impreflex-compressed-goal*      | 55651  |  341.466 |  220.180 |

* Using `lurk --rc 400 ...`
** CPU time is outdated as we can't get only CPU execution due to a bug on Lurk's
own implementation