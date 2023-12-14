# CSL Examples

## Risc 0: v0.16.1
|   Examples   |  Cycles | CPU Exec Time | GPU Exec Time | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:------------:|:-------:|:-------------:|:-------------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| perceptron   |  21156  |     0.029     |     0.017     |      2.316     |      0.639     |      0.001      |      0.001      |      2.346     |      0.657     |
| svm5         |  21156  |     0.028     |     0.028     |      2.323     |      0.599     |      0.002      |      0.001      |      2.353     |      0.628     |
| transfer5000 | 754199  |     0.057     |     0.041     |     37.970     |      7.670     |      0.001      |      0.001      |     38.028     |      7.712     |
| transfer     |  21156  |     0.029     |     0.017     |      2.336     |      0.571     |      0.002      |      0.001      |      2.367     |      0.589     |


## zkLLVM
|     Examples     | CPU Circuit Gen Time | CPU Prove+Verify Time | GPU Time |
|:----------------:|:--------------------:|:---------------------:|:--------:|
| perceptron       |                 0.95 |                 0.135 |          |
| svm5             |                 0.96 |                 0.135 |          |
| transfer         |                 0.97 |                 0.133 |          |
| transfer-batch   |                0.797 |                 40.34 |          |


## Lurk
|     Examples     |  Cycles | CPU Time** | GPU Prove Time | GPU Verify Time | GPU Total Time |
|:----------------:|:-------:|:----------:|:--------------:|:---------------:|:--------------:|
| perceptron       |    11   |            |           0.84 |           0.597 |          1.437 |
| svm5             |    9    |            |           0.82 |           0.607 |          1.427 |
| transfer         |    34   |            |          2.205 |           0.595 |          2.800 |
| transfer5000*    |  330387 |            |        487.322 |           4.899 |        492.221 |

* Using `lurk --rc 400 transfer5000.lurk`, other tests doesn't use `--rc`


## Cairo0
|     Examples     | CPU Exec Time | CPU Prove Time | CPU Verify Time | CPU Total Time |
|:-----------------|:-------------:|:--------------:|:---------------:|:---------------|
| perceptron       |         0.438 |          0.149 |           0.007 |          0.594 |
| svm5             |         0.442 |          0.175 |           0.007 |          0.624 |
| transfer         |         0.441 |          0.185 |           0.008 |          0.634 |
| transfer5000*    |         6.799 |         26.311 |           0.867 |         33.977 |


# Proof Checker

## Risc 0: v0.16.1
|             Examples            |  Cycles | CPU Exec Time | GPU Exec Time | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:-------------------------------:|:-------:|:-------------:|:-------------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| perceptron-goal                 | 3212385 |     0.071     |     0.066     |     128.392    |     28.829     |      0.006      |      0.006      |     128.469    |     28.901     |
| svm5-goal                       | 3212385 |     0.050     |     0.050     |     128.013    |     28.520     |      0.006      |      0.006      |     128.069    |     28.576     |
| transfer-batch-1k-goal          | 6722986 |     0.112     |     0.129     |     274.692    |     58.901     |      0.011      |      0.011      |     274.815    |     59.041     |
| transfer-simple-compressed-goal | 1142529 |     0.056     |     0.052     |      48.921    |     10.606     |      0.002      |      0.003      |      48.979    |     10.661     |
| transfer-task-specific          |   89319 |     0.022     |     0.031     |       4.736    |      1.128     |      0.001      |      0.001      |       4.759    |      1.160     |
| impreflex-compressed-goal       |   67460 |     0.031     |     0.031     |       4.705    |      1.097     |      0.001      |      0.002      |       4.737    |      1.130     |


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
|             Examples            | Cycles | CPU Time** | GPU Prove Time | GPU Verify Time | GPU Total Time |
|:-------------------------------:|:------:|:----------:|:--------------:|:---------------:|:--------------:|
| transfer-task-specific*         | 148870 |    861.443 |        193.836 |           4.199 |        198.035 |
| impreflex-compressed-goal*      | 55651  |    341.466 |        108.962 |           4.209 |        113.171 |
| perceptron-goal                 | 6404208|            |              ∞ |               ∞ |                |
| svm5-goal                       | 6404208|            |              ∞ |               ∞ |                |
| transfer-batch-1k-goal          |30122047|            |              ∞ |               ∞ |                |
| transfer-simple-compressed-goal | 3202986|            |              ∞ |               ∞ |                |

* Using `lurk --rc 400 ...`
** CPU time is outdated as we can't get only CPU execution due to a bug on Lurk's
own implementation
