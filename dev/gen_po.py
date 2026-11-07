#!/usr/bin/env python3
"""Writes crates/app/translations/<lang>/LC_MESSAGES/myapp.po (bundled by build.rs). Edit the table, run, commit.
Slint's default gettext context is the component name, so msgctxt is `Main`, `SettingsDialog`, `About`, `WidgetsPage`, `InputsTab`, `ListTab`, `TableTab` or `FeedbackTab` (UI) or `Tr` (strings built in Rust)."""
import os

OUT = os.path.join(os.path.dirname(__file__), "..", "crates", "app", "translations")
LANGS = ["pt_BR"]
# (context, msgid, [translation per LANGS])
T = [
    ("Tr", "Hello, {}", ["Olá, {}"]),
    ("Tr", "Version {}", ["Versão {}"]),
    ("Main", "world", ["mundo"]),
    ("Main", "Your name", ["Seu nome"]),
    ("Main", "Hello", ["Olá"]),
    ("Main", "Widgets", ["Widgets"]),
    ("SettingsDialog", "Settings", ["Configurações"]),
    ("SettingsDialog", "Appearance", ["Aparência"]),
    ("SettingsDialog", "System", ["Sistema"]),
    ("SettingsDialog", "Light", ["Claro"]),
    ("SettingsDialog", "Dark", ["Escuro"]),
    ("SettingsDialog", "System follows your desktop's light or dark mode.", ["Sistema segue o modo claro ou escuro da sua área de trabalho."]),
    ("SettingsDialog", "Language", ["Idioma"]),
    ("SettingsDialog", "About", ["Sobre"]),
    ("WidgetsPage", "Inputs", ["Entradas"]),
    ("WidgetsPage", "List", ["Lista"]),
    ("WidgetsPage", "Table", ["Tabela"]),
    ("WidgetsPage", "Feedback", ["Progresso"]),
    ("InputsTab", "Linked values", ["Valores ligados"]),
    ("InputsTab", "Enabled", ["Ativado"]),
    ("InputsTab", "Notifications", ["Notificações"]),
    ("InputsTab", "Choice", ["Escolha"]),
    ("InputsTab", "Small", ["Pequeno"]),
    ("InputsTab", "Medium", ["Médio"]),
    ("InputsTab", "Large", ["Grande"]),
    ("InputsTab", "Text", ["Texto"]),
    ("InputsTab", "Password", ["Senha"]),
    ("InputsTab", "Notes", ["Anotações"]),
    ("ListTab", "List", ["Lista"]),
    ("ListTab", "New item", ["Novo item"]),
    ("ListTab", "Add", ["Adicionar"]),
    ("ListTab", "Remove", ["Remover"]),
    ("ListTab", "No items yet. Type a name and press Add.", ["Nenhum item ainda. Digite um nome e clique em Adicionar."]),
    ("ListTab", "1 item", ["1 item"]),
    ("ListTab", "{} items", ["{} itens"]),
    ("TableTab", "Files", ["Arquivos"]),
    ("TableTab", "Name", ["Nome"]),
    ("TableTab", "Type", ["Tipo"]),
    ("TableTab", "Size (KB)", ["Tamanho (KB)"]),
    ("TableTab", "No files found", ["Nenhum arquivo encontrado"]),
    ("TableTab", "No files match the search or filters", ["Nenhum arquivo corresponde à pesquisa ou aos filtros"]),
    ("TableTab", "Search files", ["Pesquisar arquivos"]),
    ("TableTab", "Filters", ["Filtros"]),
    ("TableTab", "Filters ({})", ["Filtros ({})"]),
    ("TableTab", "All types", ["Todos os tipos"]),
    ("TableTab", "Larger than 1 MB", ["Maiores que 1 MB"]),
    ("TableTab", "Clear", ["Limpar"]),
    ("TableTab", "1 file", ["1 arquivo"]),
    ("TableTab", "{} files", ["{} arquivos"]),
    ("TableTab", "{} of {} files", ["{} de {} arquivos"]),
    ("FeedbackTab", "Task", ["Tarefa"]),
    ("FeedbackTab", "Start task", ["Iniciar tarefa"]),
    ("FeedbackTab", "Cancel", ["Cancelar"]),
    ("FeedbackTab", "Running…", ["Em andamento…"]),
    ("FeedbackTab", "Done", ["Concluída"]),
    ("FeedbackTab", "Press Start task to try it.", ["Clique em Iniciar tarefa para testar."]),
    ("FeedbackTab", "Static examples", ["Exemplos estáticos"]),
    ("SettingsDialog", "Close", ["Fechar"]),
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
