import rust

def main():
    say_hello()
    rust.say_hello()

def say_hello():
    print("Hello!  This is from python.")

if __name__ == "__main__":
    main()