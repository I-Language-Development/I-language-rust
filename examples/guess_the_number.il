import random;

print("Welcome to: Guess the number!");

int goal = random.randint(0, 25);
int guess = 0;

while (guess != goal) {
    try {
        guess = int(input("Please input a number to guess (0-25): "));
    } catch TypeError(value) {
        print(f"{value} is not a number. Using zero instead.");
    }

    if (guess <= 0 || guess > 25) {
        print(f"{guess} is not in the range of 0 to 25.");
        continue;
    }

    if (guess > goal) {
        print("The random number is lower than your guess.");
    }

    if (guess < goal) {
        print("The random number is higher than your guess.");
    }
}

print("Perfect! You guessed the number right!");
