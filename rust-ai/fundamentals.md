# Neural Networks
First lets get the mental model of what Neural Network is.

Imagine a factory responsible for producing solid metal speheres. Factory orders metal cubes of a certain size and places them into
their special cutting machine which outputs solid metal spheres.

The workers place the cubes onto and assembly line and off it goes into the machine. The whole factory loves this cutting machine but
they start to become old and rusted. The factory owner calls the company and they send you the lead engineer out to fix the machines.

You first place cube into the machine to ascertain the state of the gears. The end result is half cube, half sphere shape.

pic

Now the engineer realizes something, every layer of gears is dependent upon the layer before it. By tracing the gears we can see exactly how
each chain of gears affects the output of our machine.

pic

The engineer tweaks some gears in the chain he can see that are the most responsible for the incorrect output and turns on the machine to try
with another Cube. Success!

He realizes that the new shape is closer to a sphere but he's still off. He continues to repeat the following steps:
- **Forward Propagation**: Pushing the cube through the machine
- **Cost Function**: Where he calculates how close the output is to a perfect sphere.
- **Back Propagation**: He goes back into the gears and updates them.

This machine you may have guessed by now is a Neural Network.

pic

Let's zoom in on just one of these circles. It's called a **Neuron**, meant to loosly mirror how the brain works.

pic

Let's take a look at a two input Neuron.

# Neuron

pic

Three things are happening here. First each of our inputs is multiplied by it's appropriate weight.

pic

A weight is something that determines the strength or importance of each input signal.

Next all the weighted inputs are added together with a **Bias**.

pic

Finally, this now weighted sum is passed through an **activation function**. The reason we use an **activation function** is to make the
neuron non-linear.

pic

Think about it, if there was no activation function then the entire neural network would be simplified into a series of linear functions and
we would never be able to fit any complex data.

That's all fine in well for a two input Neuron but imagine there were hundreds or even thousands of weights and inputs to a single Neuron. How
would we solve this? 

That's where metrices come in. If we place all the inputs into a matrix and all the weights into another. We can just multiply them together
and add them to a vector of biases. Then we take our results and pass them into our activation function.

pic


