import { createHash } from 'node:crypto';

const MAGIC = 123456789;
const SIGNATURE = Buffer.from('002e0063006d003400020000', 'hex');
const hash = (buffer) => createHash('sha256').update(buffer).digest('hex');

function requireBytes(buffer, offset, size) {
  if (offset + size > buffer.length) throw new Error(`Truncated record at offset ${offset}: requires ${size} bytes`);
}

function text(buffer, start, size) {
  return buffer.subarray(start, start + size).toString('utf16le').replace(/\0+$/, '');
}

// Container layout observed in CM0304 4.1.5 samples. Payloads remain opaque.
export function readSave(buffer) {
  requireBytes(buffer, 0, 12);
  if (!buffer.subarray(0, 12).equals(SIGNATURE)) throw new Error('Unsupported CM4 signature');
  const blocks = [], records = [];
  let offset = 12;
  while (offset < buffer.length) {
    requireBytes(buffer, offset, 5);
    if (buffer.readUInt32LE(offset) !== MAGIC) throw new Error(`Invalid record marker at offset ${offset}`);
    const type = buffer[offset + 4];
    if (type === 0) {
      requireBytes(buffer, offset, 59);
      records.push({ type, offset, size: 59, name: text(buffer, offset + 5, 54) });
      offset += 59;
    } else if (type === 1) {
      records.push({ type, offset, size: 5 });
      offset += 5;
    } else if (type === 2) {
      requireBytes(buffer, offset, 73);
      const size = buffer.readUInt32LE(offset + 69);
      const payloadOffset = offset + 73;
      requireBytes(buffer, payloadOffset, size);
      const name = text(buffer, offset + 5, 54);
      const extension = text(buffer, offset + 59, 10);
      const payload = buffer.subarray(payloadOffset, payloadOffset + size);
      blocks.push({ name, extension, offset, payloadOffset, size, sha256: hash(payload) });
      records.push({ type, offset, size: 73 + size });
      offset = payloadOffset + size;
    } else throw new Error(`Unknown record type ${type} at offset ${offset}`);
  }
  return { size: buffer.length, sha256: hash(buffer), blocks, records };
}

function indexed(blocks) {
  const result = new Map();
  for (const block of blocks) {
    const key = block.name + block.extension;
    if (result.has(key)) throw new Error(`Ambiguous duplicate block: ${key}`);
    result.set(key, block);
  }
  return result;
}

export function compareSaves(before, after) {
  const left = indexed(before.blocks), right = indexed(after.blocks);
  const blocks = [...new Set([...left.keys(), ...right.keys()])].sort().map((name) => {
    const a = left.get(name), b = right.get(name);
    const status = !a ? 'added' : !b ? 'removed' : a.sha256 === b.sha256 ? 'unchanged' : 'changed';
    return { name, status, beforeSize: a?.size ?? null, afterSize: b?.size ?? null,
      delta: (b?.size ?? 0) - (a?.size ?? 0), beforeOffset: a?.offset ?? null, afterOffset: b?.offset ?? null };
  });
  const counts = { added: 0, removed: 0, changed: 0, unchanged: 0 };
  for (const block of blocks) counts[block.status]++;
  return { beforeSize: before.size, afterSize: after.size, delta: after.size - before.size, counts, blocks };
}
