import { readFile, writeFile } from 'node:fs/promises';
import { resolve, basename, extname } from 'node:path';
import { createHash } from 'node:crypto';
import { editAttribute } from '../src/save-writer.js';

async function main() {
  const [source, destination, uid, attribute, value] = process.argv.slice(2);
  if (process.argv.length !== 7) throw new Error('Usage: node tools/edit-save-cli.js <source> <new-copy> <UID> <attribute> <1-20>');
  if (extname(destination).toLowerCase() !== '.cm4') throw new Error('Destination must have a .cm4 extension');
  if (basename(destination, extname(destination)).length > 26) throw new Error('Use a save name of at most 26 characters: the game truncates longer names');
  if (resolve(source).toLowerCase() === resolve(destination).toLowerCase()) throw new Error('Destination must be a new copy');
  const input = await readFile(source);
  const { output, change } = editAttribute(input, Number(uid), attribute, Number(value));
  await writeFile(destination, output, { flag: 'wx' });
  const reread = await readFile(destination);
  if (!reread.equals(output)) throw new Error('Saved copy differs from verified buffer');
  const hash = b => createHash('sha256').update(b).digest('hex');
  if (hash(await readFile(source)) !== hash(input)) throw new Error('Source changed during operation');
  const report = { source: resolve(source), destination: resolve(destination), sourceSha256: hash(input), copySha256: hash(output), size: output.length, changedBytes: 1, change };
  await writeFile(`${destination}.edit.json`, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  console.log(JSON.stringify(report, null, 2));
}
main().catch(error => { console.error(error.message); process.exitCode = 1; });
