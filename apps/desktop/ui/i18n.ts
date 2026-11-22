import { english } from './en.js';

const languageKey = 'cm0304.language.v1';
export type Locale = 'pt-BR' | 'en-GB';

function storedLocale(): Locale {
  try {
    if (localStorage.getItem(languageKey) === 'pt-BR') return 'pt-BR';
  } catch {
    // O editor continua disponível quando o armazenamento local está indisponível.
  }
  return 'en-GB';
}

export const locale = storedLocale();

export function saveLocale(value: Locale) {
  localStorage.setItem(languageKey, value);
}

export function t(text: string): string {
  if (locale === 'pt-BR') return text;
  const original = text.trim();
  const key = original.replace(/\s+/g, ' ');
  if (!Object.hasOwn(english, key)) return text;
  const start = text.indexOf(original);
  return text.slice(0, start) + english[key] + text.slice(start + original.length);
}

export function localizeIpcError(error: unknown): unknown {
  if (locale === 'pt-BR' || typeof error !== 'string') return error;
  const detailMarker = '. Detalhe: ';
  const detailStart = error.lastIndexOf(detailMarker);
  const message = detailStart === -1 ? error : error.slice(0, detailStart);
  const detail = detailStart === -1 ? '' : error.slice(detailStart + detailMarker.length);
  const readError = /^Não foi possível ler o arquivo (.+)$/.exec(message);
  if (readError) return 'Could not read ' + readError[1] + (detail ? '. Details: ' + detail : '.');
  const openError = /^Não foi possível abrir (.+) como um save compatível\./.exec(message);
  if (openError) {
    return 'Could not open ' + openError[1] + '. Select an uncompressed .cm4 save from CM0304 4.1.5.'
      + (detail ? ' Details: ' + detail : '');
  }
  const translated = t(message);
  if (translated !== message) return translated + (detail ? '. Details: ' + detail : '');
  return 'The editor could not complete this operation. Details: ' + error;
}

export function localizeStaticUi() {
  document.documentElement.lang = locale;
  if (locale === 'pt-BR') return;
  // Aqui o corpo ainda contém apenas a interface HTML, sem dados do save.
  const nodes = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  while (nodes.nextNode()) {
    const node = nodes.currentNode;
    if (node.nodeValue) node.nodeValue = t(node.nodeValue);
  }
  for (const attribute of ['title', 'aria-label', 'placeholder']) {
    for (const element of document.querySelectorAll('[' + attribute + ']')) {
      element.setAttribute(attribute, t(element.getAttribute(attribute)!));
    }
  }
}
