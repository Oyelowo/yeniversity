# Phase 08 — Artificial Intelligence & Machine Learning

**Duration:** 3–5 months  
**Prerequisites:** Phase 01 (linear algebra, calculus, probability), Phase 04 (signals & systems for CNNs/DSP connection)  
**Can start:** After Phase 01 is solid; does not require later phases

---

## Why This Phase Exists

AI and ML are applied mathematics — linear algebra, probability, and optimisation in service of
learning from data and making decisions. Now that you have those foundations, you can build every
algorithm from scratch and understand every design decision. The goal here is not to use
frameworks — it is to implement backpropagation, attention, value iteration, and SLAM-graph
optimisation yourself in Rust, and understand *why* each works at the level of the mathematics.
Combined with robotics (Phase 06), this gives you fully autonomous physical systems.

---

## Learning Objectives

- [ ] Implement and understand linear and logistic regression as convex optimisation problems
- [ ] Derive and implement gradient descent, stochastic gradient descent, Adam; understand convergence
- [ ] Implement a neural network with forward and backward passes (backpropagation) from scratch
- [ ] Implement and understand convolutional neural networks; understand weight sharing, receptive fields
- [ ] Implement and understand recurrent neural networks (LSTM, GRU); understand vanishing gradients
- [ ] Implement and understand the attention mechanism and the Transformer architecture
- [ ] Understand and implement generalisation techniques: regularisation, dropout, batch normalisation
- [ ] Understand Bayesian machine learning: MAP estimation, Bayesian linear regression, Gaussian processes
- [ ] Understand reinforcement learning: MDPs, policy and value functions, Bellman equations
- [ ] Implement Q-learning, SARSA, DQN, and policy gradient (REINFORCE)

---

## Topics

### 1. Mathematical Foundations Review for ML

- Linear algebra: matrix factorisation in ML (SVD, PCA); projections; quadratic forms
- Multivariate calculus: chain rule; Jacobians; Hessians; second-order optimisation
- Probability: expectation; variance; MLE; MAP; KL divergence; entropy
- Information theory: entropy $H(X) = -\sum p \log p$; mutual information; cross-entropy loss
- Convexity: convex functions; convex sets; why convexity guarantees unique minimum

### 2. Classical Machine Learning

#### Supervised Learning
- Linear regression: normal equations; geometric interpretation; probabilistic interpretation as MLE
- Ridge regression (L2) and Lasso (L1): regularisation as prior; effect on bias-variance
- Bias-variance tradeoff; generalisation error; model selection; cross-validation
- Logistic regression: sigmoid function; maximum likelihood; gradient descent; Newton's method
- Binary and multi-class classification; softmax; cross-entropy loss

#### Optimisation
- Gradient descent and its variants (batch, mini-batch, stochastic)
- Momentum; Nesterov momentum; AdaGrad; RMSProp; Adam; learning rate schedules
- Second-order methods: Newton's method; BFGS (limited memory L-BFGS); when they are practical
- Saddle points and local minima in high-dimensional loss landscapes

#### Support Vector Machines
- Maximum margin classifier; support vectors; primal and dual formulations
- Lagrangian duality; KKT conditions; soft-margin SVM
- Kernel trick: RBF, polynomial kernels; feature spaces
- SVM vs. logistic regression

#### Decision Trees and Ensemble Methods
- Decision tree learning: information gain, Gini impurity, stopping criteria, pruning
- Random forests: bagging + feature randomness
- Gradient boosted trees: additive model, each tree corrects residuals; XGBoost/LightGBM concepts

#### Clustering and Dimensionality Reduction
- K-means clustering: algorithm, convergence, sensitivity to initialisation; k-means++
- Gaussian Mixture Models (GMM): EM algorithm derivation
- PCA: derivation via SVD; explained variance; covariance matrix eigendecomposition
- t-SNE: overview; use as visualisation tool only

### 3. Neural Networks

#### Feedforward Neural Networks
- Neuron model: weighted sum + activation; activation functions (sigmoid, tanh, ReLU, GELU)
- Universal approximation theorem (statement and intuition)
- Forward pass: layer-by-layer computation
- Loss functions: MSE, cross-entropy, binary cross-entropy; when to use each
- Backpropagation: chain rule applied systematically; derivation of gradient for each layer
- Weight initialisation: Xavier/Glorot; Kaiming/He; understand variance propagation

#### Training Dynamics
- Vanishing and exploding gradients; gradient clipping
- Batch normalisation: computation during training and inference; benefits
- Dropout: training vs. inference; connection to ensemble methods
- Layer normalisation; weight normalisation

#### Convolutional Neural Networks (CNNs)
- Convolution as cross-correlation in practice; filters; feature maps
- Receptive field; striding and padding; channel dimensions
- Pooling: max, average, global average pooling
- Standard architectures: LeNet → AlexNet → VGG → ResNet (residual connections) → EfficientNet
- Why residual connections solve the degradation problem
- Transposed convolution (deconvolution) for upsampling
- Depthwise separable convolutions (MobileNet)

#### Recurrent Neural Networks (RNNs)
- Vanilla RNN: hidden state; unrolling through time; backpropagation through time (BPTT)
- Vanishing gradient problem in deep time steps
- Long Short-Term Memory (LSTM): cell state; forget, input, output gates; derivation
- Gated Recurrent Unit (GRU): simplified gating mechanism
- Sequence-to-sequence models; encoder-decoder architecture

#### Attention Mechanism and Transformers
- Scaled dot-product attention: queries, keys, values; softmax scoring; output as weighted sum
- Multi-head attention: learning multiple attention patterns in parallel
- Positional encoding: sinusoidal encoding; why position information is needed
- Transformer encoder: attention + feed-forward + layer norm + residual
- Transformer decoder: autoregressive; masked self-attention + cross-attention
- Pre-norm vs. post-norm; modern architectural variants (PaLM, LLaMA, etc.)
- Vision Transformer (ViT): patch embedding for images
- Training transformers: tokenisation, vocabulary, next-token prediction loss (language modelling)

### 4. Bayesian Machine Learning

- Bayesian inference recap: prior, likelihood, posterior
- Bayesian linear regression: conjugate Gaussian prior; closed-form posterior
- Predictive distribution; uncertainty; epistemic vs. aleatoric uncertainty
- Gaussian Processes (GP): GP as distribution over functions; covariance kernels (RBF, Matérn)
- GP regression: posterior mean and variance; hyperparameter optimisation via MLE of marginal likelihood
- GP for Bayesian optimisation: acquisition functions (UCB, Expected Improvement)
- Variational inference: ELBO; mean-field approximation; VI vs. MCMC trade-offs

### 5. Reinforcement Learning

#### Fundamentals
- Agent, environment, state, action, reward, policy, value function
- Markov decision process (MDP): states, actions, transition model $P(s'|s,a)$, reward $R(s,a)$, discount $\gamma$
- Policy $\pi(a|s)$; state value function $V^\pi(s)$; action value function $Q^\pi(s,a)$
- Bellman expectation equations: $V^\pi(s) = \sum_a \pi(a|s)[R + \gamma \sum_{s'} P V^\pi(s')]$
- Optimal policy; Bellman optimality equations; $V^*$ and $Q^*$

#### Dynamic Programming (Model-Based)
- Policy evaluation: iterative solution to Bellman expectation equation
- Policy improvement: greedy policy; policy iteration
- Value iteration: direct iteration on Bellman optimality equation
- Generalised policy iteration framework

#### Model-Free Prediction and Control
- Monte Carlo methods: episode returns; first-visit and every-visit MC
- Temporal difference learning: TD(0); TD error $\delta_t = r + \gamma V(s') - V(s)$
- SARSA (on-policy TD control)
- Q-learning (off-policy TD control): $Q \leftarrow Q + \alpha[r + \gamma \max_{a'} Q(s', a') - Q]$
- Eligibility traces: TD(λ); connecting MC and TD

#### Deep Reinforcement Learning
- Deep Q-Networks (DQN): experience replay; target network; deadly triad
- Double DQN; Dueling DQN
- Policy gradient theorem; REINFORCE algorithm
- Actor-Critic: A2C, A3C, SAC (soft actor-critic), PPO (proximal policy optimisation)
- Model-based RL overview: Dyna, planning with learned models

---

## Core Texts

| Text | Notes |
|------|-------|
| **Bishop — Pattern Recognition and Machine Learning** | The Bayesian ML bible. Dense but complete. Work through chapters 1–8 for core ML. |
| **Goodfellow, Bengio & Courville — Deep Learning** | Free at deeplearningbook.org. The standard deep learning reference. Parts I and II are essential. |
| **Sutton & Barto — Reinforcement Learning: An Introduction** (2nd ed.) | Free online. The RL bible. Work through the exercises systematically. |
| **Murphy — Probabilistic Machine Learning** (Vol. 1 & 2) | Free online. Contemporary, rigorous, Bayesian perspective. Excellent reference. |

---

## Supplementary
- Karpathy's *Neural Networks: Zero to Hero* (YouTube) — backprop and transformers from scratch
- Andrej Karpathy's *makemore* and *nanoGPT* — small-scale transformer implementation to study
- *Understanding Deep Learning* — Simon Prince (free online) — recent comprehensive text
- MIT 6.S191 *Introduction to Deep Learning* (OCW, free) — quick practical survey

---

## Rust Simulation Projects

*All implementations should be from scratch — no PyTorch, no ndarray magic. Use nalgebra for matrix arithmetic.*

| Project | Description |
|---------|-------------|
| Linear regression (gradient descent) | With L1/L2 regularisation; plot loss curve; compare to closed-form |
| Feedforward neural net from scratch | Forward + backward pass; train on XOR and MNIST |
| CNN from scratch | Convolution, pooling, backprop through conv layers; train on MNIST |
| LSTM from scratch | Implement gates; train on character-level sequence modelling |
| Transformer (tiny) | Scaled dot-product attention; multi-head; positional encoding; train on toy task |
| Value iteration | MDP gridworld; verify convergence to optimal policy |
| Q-learning | Tabular Q-learning on GridWorld and simple custom environments |
| Gaussian Process | GP regression with RBF kernel; posterior mean and variance; hyperparameter fitting |

---

## Completion Criteria

You are ready to move on when you can:
1. Implement backpropagation for a multi-layer network manually (on paper, then in Rust), deriving every gradient
2. Explain why residual connections help training; what the gradients look like without them
3. Derive the attention mechanism from the query/key/value perspective
4. Write down the Bellman optimality equation and derive Q-learning from it
5. Implement Gaussian process regression in Rust with a kernel of your choice
