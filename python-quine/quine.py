x = "y = \"x = \\\"\"\nfor i in range(len(x)) :\n    c = x[i]\n    if c == '\\\"' :\n        y += \"\\\\\\\"\"\n    elif c == '\\n' :\n        y += \"\\\\n\"\n    elif c == '\\\\' :\n        y += \"\\\\\\\\\"\n    else :\n        y += c\ny += \"\\\"\"\nprint(y + \"\\n\" + x)"
y = "x = \""
for i in range(len(x)) :
    c = x[i]
    if c == '\"' :
        y += "\\\""
    elif c == '\n' :
        y += "\\n"
    elif c == '\\' :
        y += "\\\\"
    else :
        y += c
y += "\""
print(y + "\n" + x)
