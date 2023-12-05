# CSL Examples

## Risc 0
|     Examples     |  Cycles | CPU Time | GPU Time |
|:----------------:|:-------:|:--------:|:--------:|
| perceptron       |  21156  |   2.359  |   0.622  |
| svm5             |  21156  |   2.641  |   0.621  |
| transfer5000     | 724258  |  37.879  |   7.586  |
| transfer         |  21156  |   2.333  |   0.597  |
		

## zkLLVM
|     Examples     | CPU Time | GPU Time |
|:----------------:|:--------:|:--------:|
| perceptron       |   0.198  |          |
| svm5             |   0.197  |          |
| transfer5000     |  43.994  |          |
| transfer         |   0.198  |          |


## Lurk
|     Examples     |  Cycles | CPU Time | GPU Time |
|:----------------:|:-------:|:--------:|:--------:|
| perceptron       |    11   |   3.979  |          |
| svm5             |    9    |   2.263  |          |
| transfer5000*    |  330387 | 1766.207 |          |
| transfer         |    34   |   2.522  |          |


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
|             Examples            | CPU Time | GPU Time |
|:-------------------------------:|:--------:|:--------:|
| perceptron-goal                 |     ∞    |          |
| svm5-goal                       |     ∞    |          |
| transfer-batch-1k-goal          |     ∞    |          |
| transfer-simple-compressed-goal | 8066.663 |          |
| transfer-task-specific          |  878.184 |          |
| impreflex-compressed-goal       |  417.277 |          |

## Lurk
|             Examples            | Cycles | CPU Time | GPU Time |
|:-------------------------------:|:------:|:--------:|:--------:|
| perceptron-goal                 | 6404208|     ∞    |          |
| svm5-goal                       | 6404208|     ∞    |          |
| transfer-batch-1k-goal          |30122047|     ∞    |          |
| transfer-simple-compressed-goal | 3202986|     ∞    |          |
| transfer-task-specific*         | 148870 |  861.443 |          |
| impreflex-compressed-goal*      | 55651  |  341.466 |          |

* Using `lurk --rc 400 ...`
