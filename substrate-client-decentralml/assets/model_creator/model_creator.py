import os
import glob

import numpy as np
import click

import tensorflow as tf

os.environ["TF_CPP_MIN_LOG_LEVEL"] = "3"

from settings import MODEL_PATH, CONTRIBUTORS_MODELS_PATH, MODEL_NAME

@click.group()
def cli():
    pass

def create_model():
    # Load model and data (MobileNetV2, CIFAR-10)
    model = tf.keras.models.Sequential([
                tf.keras.layers.Flatten(input_shape=(28, 28)),
                tf.keras.layers.Dense(128, activation='relu'),
                tf.keras.layers.Dense(10)
                ])
    model.compile(
        optimizer=tf.keras.optimizers.Adam(0.001),
        loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
        metrics=[tf.keras.metrics.SparseCategoricalAccuracy()],
    )
    return model

def load_model(model_path, model_name):
    model = tf.keras.models.load_model(f"{model_path}/{model_name}")
    print(model.summary())
    return model

def load_data():
    (x_train, y_train), (x_test, y_test) = tf.keras.datasets.mnist.load_data()
    x_train, x_test = x_train / 255.0, x_test / 255.0
    return (x_train, y_train), (x_test, y_test)

def train_model(model, x_train, y_train, epochs=1):
    model.fit(x_train, y_train, epochs)
    return model

def evaluate_model(model, x_test, y_test):
    return model.evaluate(x_test, y_test)

def save_model(model, output_path, model_name):
    model.save(f"{output_path}/{model_name}")

def load_contributors_models(contributors_models_path):
    model_contributors_folders_path = f"{contributors_models_path}{MODEL_NAME}_*"
    model_folders = [f for f in glob.glob(model_contributors_folders_path)]
    contributors_models = list()
    for model_folder in model_folders:
        model = tf.keras.models.load_model(model_folder)
        contributors_models.append(model)
    return contributors_models

def old_federate_contributors_model(contributors_models, policy="average"):
    contributors_weights = [model.get_weights() for model in contributors_models]
    new_weights = list()
    if policy=="average":
        for weights_list_tuple in zip(*contributors_weights):
            new_weights.append(
                [np.array(weights_).mean(axis=0) for weights_ in zip(*weights_list_tuple)])
    return new_weights

def federate_contributors_model(contributors_models, policy="average"):
    # Retrieve the trainable variables from each client model
    client_weights = [model.trainable_variables for model in contributors_models]

    # Compute the average weights for each layer
    avg_weights = [
        tf.reduce_mean(layer_weight_tensors, axis=0)
        for layer_weight_tensors in zip(*client_weights)
    ]

    return avg_weights


def set_model_weights(model, weights):
    model.set_weights(weights)
    return model

@click.command()
def create_new_model():
    model = create_model()
    save_model(model, MODEL_PATH, MODEL_NAME)

@click.command()
def federate_contributors_model():
    (x_train, y_train), (x_test, y_test) = load_data()
    contributors_models = load_contributors_models(CONTRIBUTORS_MODELS_PATH)
    for contr_model in contributors_models:
        print(contr_model.summary())
    old_model = load_model(MODEL_PATH, MODEL_NAME)
    new_weights = federate_contributors_model(contributors_models)
    new_model = set_model_weights(old_model, new_weights)
    print(evaluate_model(new_model, x_test, y_test))

cli.add_command(create_new_model)
cli.add_command(federate_contributors_model)

if __name__ == "__main__":
    cli()