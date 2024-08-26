#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define INPUT_NODES 2
#define HIDDEN_NODES 2
#define OUTPUT_NODES 1
#define LEARNING_RATE 0.1

double sigmoid(double x) { return 1 / (1 + exp(-x)); }
double d_sigmoid(double x) { return x * (1 - x); }

void forward_pass(double input[], double hidden_weights[][HIDDEN_NODES], double output_weights[], double hidden_layer[], double *output) {
    for (int i = 0; i < HIDDEN_NODES; i++) {
        hidden_layer[i] = sigmoid(input[0] * hidden_weights[0][i] + input[1] * hidden_weights[1][i]);
    }
    *output = sigmoid(hidden_layer[0] * output_weights[0] + hidden_layer[1] * output_weights[1]);
}

void train(double input[], double hidden_weights[][HIDDEN_NODES], double output_weights[], double hidden_layer[], double *output, double target) {
    forward_pass(input, hidden_weights, output_weights, hidden_layer, output);

    double output_error = (target - *output) * d_sigmoid(*output);
    double hidden_errors[HIDDEN_NODES];

    for (int i = 0; i < HIDDEN_NODES; i++) {
        hidden_errors[i] = output_error * output_weights[i] * d_sigmoid(hidden_layer[i]);
    }

    for (int i = 0; i < HIDDEN_NODES; i++) {
        output_weights[i] += LEARNING_RATE * output_error * hidden_layer[i];
        for (int j = 0; j < INPUT_NODES; j++) {
            hidden_weights[j][i] += LEARNING_RATE * hidden_errors[i] * input[j];
        }
    }
}

int main() {
    double hidden_weights[INPUT_NODES][HIDDEN_NODES] = {{0.15, 0.25}, {0.20, 0.30}};
    double output_weights[HIDDEN_NODES] = {0.40, 0.50};
    double hidden_layer[HIDDEN_NODES], output;

    double training_data[4][3] = {{0, 0, 0}, {0, 1, 1}, {1, 0, 1}, {1, 1, 0}};

    for (int epoch = 0; epoch < 10000; epoch++) {
        for (int i = 0; i < 4; i++) {
            train(training_data[i], hidden_weights, output_weights, hidden_layer, &output, training_data[i][2]);
        }
    }

    for (int i = 0; i < 4; i++) {
        forward_pass(training_data[i], hidden_weights, output_weights, hidden_layer, &output);
        printf("Input: %.0f %.0f, Output: %f\n", training_data[i][0], training_data[i][1], output);
    }

    return 0;
}
