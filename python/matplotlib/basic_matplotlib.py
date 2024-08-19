import matplotlib.pyplot as plt

def plot_data(x, y):
    plt.plot(x, y, marker='o')
    plt.title('Sample Plot')
    plt.xlabel('X-axis')
    plt.ylabel('Y-axis')
    plt.show()

# Usage example
x = [1, 2, 3, 4, 5]
y = [2, 3, 5, 7, 11]
plot_data(x, y)
