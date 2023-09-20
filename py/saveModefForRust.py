import tensorflow as tf

model = tf.keras.models.load_model('models/model140.keras')
directory = '../model'

model.layers[0]._name = 'input_layer'
model.layers[2]._name = 'output_layer'

model.summary()

model.save(directory, save_format='tf')

