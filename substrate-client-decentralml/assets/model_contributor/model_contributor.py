import tensorflow as tf
import string
import random

from settings import MODEL_NAME, MODEL_PATH, CONTRIBUTORS_MODELS_PATH


def load_model(model_path, model_name):
    model = tf.keras.models.load_model(f"{model_path}/{model_name}")
    print(model.summary())
    return model

def load_data():
    (x_train, y_train), (x_test, y_test) = tf.keras.datasets.mnist.load_data()
    x_train, x_test = x_train / 255.0, x_test / 255.0
    return (x_train, y_train), (x_test, y_test)

def train_model(model, x_train, y_train, epoch=100, batch_size=32):
    model.fit(x_train, y_train, epochs=epoch, batch_size=batch_size)
    return model

def evaluate_model(model, x_test, y_test):
    print(model.evaluate(x_test, y_test))
    # return loss, len(x_test), {"accuracy": accuracy}

def get_random_string(length):
    # choose from all lowercase letter
    letters = string.ascii_lowercase
    result_str = ''.join(random.choice(letters) for i in range(length))
    return result_str

def save_model(model, output_path, model_name, contributo_id_length=10):
    contributor_id = get_random_string(contributo_id_length)
    model.save(f"{output_path}/{model_name}_{contributor_id}")

if __name__ == "__main__":
    model = load_model(MODEL_PATH, MODEL_NAME)
    (x_train, y_train), (x_test, y_test) = load_data()
    model = train_model(model, x_train, y_train)
    evaluate_model(model, x_test, y_test)
    save_model(model, CONTRIBUTORS_MODELS_PATH, MODEL_NAME)



