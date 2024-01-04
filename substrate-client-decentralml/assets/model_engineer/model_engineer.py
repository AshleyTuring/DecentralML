import os
import string
import random

import tensorflow as tf

os.environ["TF_CPP_MIN_LOG_LEVEL"] = "3"

from settings import MODEL_PATH, CONTRIBUTORS_MODELS_PATH, MODEL_NAME

def redefine_model():
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

def get_random_string(length):
    # choose from all lowercase letter
    letters = string.ascii_lowercase
    result_str = ''.join(random.choice(letters) for i in range(length))
    return result_str

def save_model(model, output_path, model_name, contributo_id_length=10):
    contributor_id = get_random_string(contributo_id_length)
    model.save(f"{output_path}/{model_name}_{contributor_id}")

def set_model_weights(model, weights):
    model.set_weights(weights)
    return model

if __name__ == "__main__":
    pass