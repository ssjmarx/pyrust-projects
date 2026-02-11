import rust

def main():
    say_hello()
    # print(rust.say_hello(get_secret_word()))

    print(rust.multilingual_greeting(get_preferred_language()))

def get_preferred_language():
    print("Select your language / Selecciona tu idioma / Sélectionnez votre langue / Wählen Sie Ihre Sprache")
    print("1. English")
    print("2. Español")
    print("3. Français")
    print("4. Deutsch")

    selection = input("Enter your choice (1-4): ")

    match selection:
        case "1":
            return "English"
        case "2":
            return "Spanish"
        case "3":
            return "French"
        case "4":
            return "German"
        case _:
            print("Eraro, bonvolu provi denove")
            return get_preferred_language()

def say_hello():
    print("Hello!  This is from python.")

def get_secret_word():
    print("Enter a secret word.")
    secret_word = input()
    return secret_word

if __name__ == "__main__":
    main()