'''
ᚪ - 'a'
ᛒ - 'b'
ᚳ - 'ch'
ᛞ - 'd'
ᛖ - 'e'
ᚠ - 'f'
ᚷ - 'g'
ᚻ - 'h'
ᛁ - 'i'
ᛡ - 'j'
ᛣ - 'k' (also might be used as 'c')
ᛚ - 'l'
ᛗ - 'm'
ᚾ - 'n'
ᚩ - 'o'
ᛈ - 'p'
ᛢ - 'q'
ᚱ - 'r'
ᛋ - 's'
ᛏ - 't'
ᚢ - 'u'
ᚹ - 'w'
ᛉ - 'x'
ᚣ - 'y'
ᛇ - 'ai'
ᚫ - 'ae'
ᚦ - 'th'
ᛝ - 'ng'
ᛠ - 'ea'
ᛟ - 'oe'
• - ' '
'''
import re
import nltk
from nltk.corpus import cmudict
from nltk.corpus import wordnet

nltk.download('wordnet', quiet=True)
nltk.download('cmudict', quiet=True)



replacements = [
    ('ᚪ', 'a'),
    ('ᛒ', 'b'),
    ('ᚳ', 'ch'),
    ('ᛞ', 'd'),
    ('ᛖ', 'e'),
    ('ᚠ', 'f'),
    ('ᚷ', 'g'),
    ('ᚻ', 'h'),
    ('ᛁ', 'i'),
    ('ᛡ', 'j'),
    ('ᛣ', 'k'),
    ('ᛚ', 'l'),
    ('ᛗ', 'm'),
    ('ᚾ', 'n'),
    ('ᚩ', 'o'),
    ('ᛈ', 'p'),
    ('ᛢ', 'q'),
    ('ᚱ', 'r'),
    ('ᛋ', 's'),
    ('ᛏ', 't'),
    ('ᚢ', 'u'),
    ('ᚹ', 'w'),
    ('ᛉ', 'x'),
    ('ᚣ', 'y'),
    ('ᛇ', 'ai'),
    ('ᚫ', 'ae'),
    ('ᚦ', 'th'),
    ('ᛝ', 'ng'),
    ('ᛠ', 'ea'),
    ('ᛟ', 'oe'),
    ('•', ' '),
]
def replace_all(match):
    word = match.group(0)
    key = re.sub(r'\W+','', word)
    return word


text = re.sub(r'\b\w+\b', replace_all, input('Text to convert:\n'))
for old, new in replacements:
    text = text.replace(old, new)

def apply_replacements(text):
    for old, new in replacements:
        pattern = re.compile(re.escape(old), re.IGNORECASE)
        def repl(m):
            return m.group(), new
        text = pattern.sub(repl, text)
    return text

text = apply_replacements(re.sub(r'\b\w+\b', replace_all, text))

print(f"\nYour Output is:\n{text} \n\nNote: You may need to swap 'c' and 'k' around.\n")
