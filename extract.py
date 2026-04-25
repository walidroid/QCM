import zipfile
import xml.etree.ElementTree as ET

def get_docx_text(path):
    document = zipfile.ZipFile(path)
    xml_content = document.read('word/document.xml')
    document.close()
    tree = ET.XML(xml_content)
    
    # Word namespace
    WORD_NAMESPACE = '{http://schemas.openxmlformats.org/wordprocessingml/2006/main}'
    PARA = WORD_NAMESPACE + 'p'
    TEXT = WORD_NAMESPACE + 't'
    
    paragraphs = []
    for paragraph in tree.iter(PARA):
        texts = [node.text
                 for node in paragraph.iter(TEXT)
                 if node.text]
        if texts:
            paragraphs.append(''.join(texts))
            
    return paragraphs

try:
    text = get_docx_text('QCM _ Algorithmique et Programmation en Python.docx')
    with open('output.txt', 'w', encoding='utf-8') as f:
        for p in text:
            f.write(p + '\n')
    print("Success")
except Exception as e:
    print("Error:", e)
