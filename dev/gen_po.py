#!/usr/bin/env python3
"""Writes crates/app/translations/<lang>/LC_MESSAGES/myapp.po (bundled by build.rs). Edit the table, run, commit.
Slint's default gettext context is the component name, so msgctxt is `Main`, `About` or `WidgetsPage` (UI) or `Tr` (strings built in Rust)."""
import os

OUT = os.path.join(os.path.dirname(__file__), "..", "crates", "app", "translations")
LANGS = ["pt_BR"]
# (context, msgid, [translation per LANGS])
T = [
    ("Tr", "Hello, {}", ["Olá, {}"]),
    ("Tr", "Version {}", ["Versão {}"]),
    ("Main", "world", ["mundo"]),
    ("Main", "Your name", ["Seu nome"]),
    ("Main", "About", ["Sobre"]),
    ("Main", "Hello", ["Olá"]),
    ("Main", "Widgets", ["Widgets"]),
    ("WidgetsPage", "Linked values", ["Valores ligados"]),
    ("WidgetsPage", "Enabled", ["Ativado"]),
    ("WidgetsPage", "Choice", ["Escolha"]),
    ("WidgetsPage", "Small", ["Pequeno"]),
    ("WidgetsPage", "Medium", ["Médio"]),
    ("WidgetsPage", "Large", ["Grande"]),
    ("WidgetsPage", "List", ["Lista"]),
    ("WidgetsPage", "New item", ["Novo item"]),
    ("WidgetsPage", "Add", ["Adicionar"]),
    ("WidgetsPage", "No items yet", ["Nenhum item ainda"]),
    ("WidgetsPage", "1 item", ["1 item"]),
    ("WidgetsPage", "{} items", ["{} itens"]),
    ("About", "A light and fast Linux app built with Rust and Slint.", ["Um app Linux leve e rápido feito com Rust e Slint."]),
    ("About", "Website", ["Site"]),
    ("About", "Close", ["Fechar"]),
]

q = lambda s: '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'
for i, lang in enumerate(LANGS):
    d = f"{OUT}/{lang}/LC_MESSAGES"
    os.makedirs(d, exist_ok=True)
    with open(f"{d}/myapp.po", "w") as f:
        f.write(f'msgid ""\nmsgstr ""\n"Content-Type: text/plain; charset=UTF-8\\n"\n"Language: {lang}\\n"\n')
        for ctx, msgid, tr in T:
            assert len(tr) == len(LANGS) and tr[i].count("{}") == msgid.count("{}"), msgid
            f.write(f"\nmsgctxt {q(ctx)}\nmsgid {q(msgid)}\nmsgstr {q(tr[i])}\n")
