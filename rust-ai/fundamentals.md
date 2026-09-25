# Neural Networks
First lets get the mental model of what Neural Network is.

Imagine a factory responsible for producing solid metal speheres. Factory orders metal cubes of a certain size and places them into
their special cutting machine which outputs solid metal spheres.

The workers place the cubes onto and assembly line and off it goes into the machine. The whole factory loves this cutting machine but
they start to become old and rusted. The factory owner calls the company and they send you the lead engineer out to fix the machines.

You first place cube into the machine to ascertain the state of the gears. The end result is half cube, half sphere shape.

<img width="777" height="497" alt="Screenshot 2026-09-25 at 10 12 57 AM" src="https://github.com/user-attachments/assets/73d70dff-6648-4389-9839-b2a5a14f0f90" />

Now the engineer realizes something, every layer of gears is dependent upon the layer before it. By tracing the gears we can see exactly how
each chain of gears affects the output of our machine.

<img width="1196" height="268" alt="Screenshot 2026-09-25 at 10 17 24 AM" src="https://github.com/user-attachments/assets/2490dbfa-4a9b-436f-aa50-3fc0b4bfb68f" />

The engineer tweaks some gears in the chain he can see that are the most responsible for the incorrect output and turns on the machine to try
with another Cube. Success!

He realizes that the new shape is closer to a sphere but he's still off. He continues to repeat the following steps:
- **Forward Propagation**: Pushing the cube through the machine
- **Cost Function**: Where he calculates how close the output is to a perfect sphere.
- **Back Propagation**: He goes back into the gears and updates them.

This machine you may have guessed by now is a Neural Network.

<img width="844" height="569" alt="Screenshot 2026-09-25 at 10 35 11 AM" src="https://github.com/user-attachments/assets/a137a0e6-0011-4d1f-89a5-581e56040882" />

Let's zoom in on just one of these circles. It's called a **Neuron**, meant to loosly mirror how the brain works.

<img width="1125" height="536" alt="Screenshot 2026-09-25 at 10 36 32 AM" src="https://github.com/user-attachments/assets/a92e8abc-8a35-4e6f-a1df-49b83732483e" />

Let's take a look at a two input Neuron.

# Neuron

<img width="847" height="352" alt="Screenshot 2026-09-25 at 10 37 32 AM" src="https://github.com/user-attachments/assets/e9137eea-2e98-48b0-8466-7bba43e9e80c" />

Three things are happening here. First each of our inputs is multiplied by it's appropriate weight.

<img width="971" height="494" alt="Screenshot 2026-09-25 at 10 38 53 AM" src="https://github.com/user-attachments/assets/78cfd039-a5bc-445a-a959-f18ccafe69aa" />

A weight is something that determines the strength or importance of each input signal.

Next all the weighted inputs are added together with a **Bias**.

<img width="786" height="565" alt="Screenshot 2026-09-25 at 10 40 32 AM" src="https://github.com/user-attachments/assets/c8ab495a-d96c-4b83-ab94-98c410367536" />

Finally, this now weighted sum is passed through an **activation function**. The reason we use an **activation function** is to make the
neuron non-linear.

<img width="698" height="486" alt="Screenshot 2026-09-25 at 10 42 32 AM" src="https://github.com/user-attachments/assets/9f536420-0cd9-436e-812a-7b827d8bf248" />

Think about it, if there was no activation function then the entire neural network would be simplified into a series of linear functions and
we would never be able to fit any complex data.

That's all fine in well for a two input Neuron but imagine there were hundreds or even thousands of weights and inputs to a single Neuron. How
would we solve this? 

That's where metrices come in. If we place all the inputs into a matrix and all the weights into another. We can just multiply them together
and add them to a vector of biases. Then we take our results and pass them into our activation function.

<img width="864" height="238" alt="Screenshot 2026-09-25 at 11 09 01 AM" src="https://github.com/user-attachments/assets/170833e7-8cb4-4e04-a2ed-1046d01aacbe" />

