#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

#define INPUT_NEURONS 2
#define HIDDEN_NEURONS 2
#define OUTPUT_NEURONS 1
#define LEARNING_RATE 0.1
#define EPOCHS 10000

double sigmoid(double x) {
    return 1 / (1 + exp(-x));
}

double sigmoid_derivative(double x) {
    return x * (1 - x);
}

void initialize_weights(double weights[], int size) {
    for (int i = 0; i < size; i++) {
        weights[i] = ((double)rand() / RAND_MAX) * 2 - 1;
    }
}

void forward_pass(double input[], double hidden_weights[], double output_weights[],
double hidden_layer[], double *output) {
    for (int i = 0; i < HIDDEN_NEURONS; i++) {
        double sum = 0;
        for (int j = 0; j < INPUT_NEURONS; j++) {
            sum += input[j] * hidden_weights[j * HIDDEN_NEURONS + i];
        }
        hidden_layer[i] = sigmoid(sum);
    }

    double sum = 0;
    for (int i = 0; i < HIDDEN_NEURONS; i++) {
        sum += hidden_layer[i] * output_weights[i];
    }
    *output = sigmoid(sum);
}

void backpropagation(double input[], double hidden_weights[], double output_weights[], 
double hidden_layer[], double output, double target) {
    double output_error = (target - output) * sigmoid_derivative(output);
    double hidden_errors[HIDDEN_NEURONS];

    for (int i = 0; i < HIDDEN_NEURONS; i++) {
        hidden_errors[i] = output_error * output_weights[i] * sigmoid_derivative(hidden_layer[i]);
    }

    for (int i = 0; i < HIDDEN_NEURONS; i++) {
        output_weights[i] += LEARNING_RATE * output_error * hidden_layer[i];
        for (int j = 0; j < INPUT_NEURONS; j++) {
            hidden_weights[j * HIDDEN_NEURONS + i] += LEARNING_RATE * hidden_errors[i] * input[j];
        }
    }
}

int main() {
    srand(time(NULL));

    double hidden_weights[INPUT_NEURONS * HIDDEN_NEURONS];
    double output_weights[HIDDEN_NEURONS];
    double hidden_layer[HIDDEN_NEURONS];
    double output;

    initialize_weights(hidden_weights, INPUT_NEURONS * HIDDEN_NEURONS);
    initialize_weights(output_weights, HIDDEN_NEURONS);

    double training_data[4][3] = {{0, 0, 0}, {0, 1, 1}, {1, 0, 1}, {1, 1, 0}};

    for (int epoch = 0; epoch < EPOCHS; epoch++) {
        double total_error = 0;
        for (int i = 0; i < 4; i++) {
            forward_pass(training_data[i], hidden_weights, output_weights, hidden_layer, &output);
            backpropagation(training_data[i], hidden_weights, output_weights, hidden_layer, output, training_data[i][2]);
            total_error += pow(training_data[i][2] - output, 2);
        }
        if (epoch % 1000 == 0) {
            printf("Epoch %d, Error: %f\n", epoch, total_error);
        }
    }

    printf("\nTesting the trained network:\n");
    for (int i = 0; i < 4; i++) {
        forward_pass(training_data[i], hidden_weights, output_weights, hidden_layer, &output);
        printf("Input: %.0f %.0f, Output: %f, Expected: %.0f\n", training_data[i][0], training_data[i][1], output, training_data[i][2]);
    }

    return 0;
}
