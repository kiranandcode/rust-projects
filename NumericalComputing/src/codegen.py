from pygments import highlight
from pygments.lexers.rust import RustLexer
from pygments.formatters import LatexFormatter

root = 'secant'
with open('./'  + root + '_temp.rs', 'r') as f:
    text = f.read()
    result = highlight(text, RustLexer(), LatexFormatter(mathescape=True))
    with open('./' + root + '.tex', 'w') as out:
        out.write(result)