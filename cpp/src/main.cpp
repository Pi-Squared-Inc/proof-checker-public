#include "../inputs/input.hpp"
#include "lib.hpp"
#include <array>
#include <iostream>

int main() {
  int result = -1;

  // Impreflex Compressed Goal
  result =
      Pattern::verify(assumption_impreflex_compressed,
                      claim_impreflex_compressed, proof_impreflex_compressed);
  std::cout << "Impeflex Compressed Goal result: " << result << std::endl;
  std::cout << std::endl;

  // Transfer Task Specific
  result = Pattern::verify(assumption_transfer_task_specific,
                           claim_transfer_task_specific,
                           proof_transfer_task_specific);
  std::cout << "Transfer Task Specific Goal result: " << result << std::endl;
  std::cout << std::endl;

  // Transfer Simple Compressed Goal
  result = Pattern::verify(assumption_transfer_simple_compressed,
                           claim_transfer_simple_compressed,
                           proof_transfer_simple_compressed);
  std::cout << "Transfer Simple Compressed Goal result: " << result
            << std::endl;
  std::cout << std::endl;

  // Transfer Batch 1k Goal
  result = Pattern::verify(assumption_transfer_batch_1k,
                           claim_transfer_batch_1k, proof_transfer_batch_1k);
  std::cout << "Transfer Batch 1k Goal result: " << result << std::endl;
  std::cout << std::endl;

  // Perceptron Goal
  result = Pattern::verify(assumption_perceptron, claim_perceptron,
                           proof_perceptron);
  std::cout << "Perceptron Goal result: " << result << std::endl;
  std::cout << std::endl;

  // Svm5 Goal
  result = Pattern::verify(assumption_svm5, claim_svm5, proof_svm5);
  std::cout << "Svm5 Goal result: " << result << std::endl;
  std::cout << std::endl;

  return 0;
}
