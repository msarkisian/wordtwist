with open("../words_unfiltered.txt", "r") as original_file:
    words = original_file.read().splitlines()

filtered_words = [word for word in words if word.isalpha()]

with open("../words.txt", "w") as filtered_file:
    for word in filtered_words:
        filtered_file.write(word + "\n")
