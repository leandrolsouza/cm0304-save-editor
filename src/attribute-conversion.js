// CM Scout 3.10 HighConvert/LowConvert and field groups; see research/FORMAT.md.
const OUTFIELD = ['crossing', 'dribbling', 'finishing', 'heading', 'longShots', 'marking', 'offTheBall', 'passing', 'penalties', 'tackling', 'creativity'];
const GOALKEEPING = ['handling', 'aerialAbility', 'commandOfArea', 'communication', 'kicking', 'throwing'];
const ALWAYS_HIGH = ['anticipation', 'decisions', 'oneOnOnes', 'positioning', 'reflexes', 'throwIns', 'firstTouch', 'technique'];

export const RELATIVE_ATTRIBUTES = [...OUTFIELD, ...GOALKEEPING, ...ALWAYS_HIGH];

export function relativeMode(attribute, goalkeeper) {
  if (OUTFIELD.includes(attribute)) return goalkeeper >= 15;
  if (GOALKEEPING.includes(attribute)) return goalkeeper < 15;
  if (ALWAYS_HIGH.includes(attribute)) return false;
  return null;
}

// Enumerate the entire signed-byte domain because rounding makes the inverse
// non-unique. Keep the nearest original encoding that displays exactly the target.
export function encodeRelativeAttribute(value, currentAbility, low, originalRaw) {
  if (!Number.isInteger(value) || value < 1 || value > 20) throw new Error('Attribute value must be an integer from 1 to 20');
  convertRelativeAttribute(originalRaw, currentAbility, low);
  let best;
  for (let raw = -128; raw <= 127; raw++) {
    if (convertRelativeAttribute(raw, currentAbility, low) !== value) continue;
    if (best === undefined || Math.abs(raw - originalRaw) < Math.abs(best - originalRaw)) best = raw;
  }
  if (best === undefined) throw new Error(`Target ${value} is not representable at CA ${currentAbility}`);
  return best;
}

export function convertRelativeAttribute(raw, currentAbility, low = false) {
  if (!Number.isInteger(raw) || raw < -128 || raw > 127) throw new Error('Expected signed attribute byte');
  if (!Number.isInteger(currentAbility) || currentAbility < 0 || currentAbility > 200) throw new Error('Expected CA between 0 and 200');
  const intermediate = ((raw * 2 + currentAbility * (low ? 0.1 : 1)) * 0.05) + 10;
  const converted = ((intermediate * intermediate * 0.1 + intermediate) / 3) + 0.5;
  return Math.max(1, Math.min(20, Math.trunc(converted)));
}

export function convertAttributes(rawAttributes, currentAbility) {
  const result = { ...rawAttributes };
  const goalkeeper = rawAttributes.goalkeeper >= 15;
  for (const name of OUTFIELD) result[name] = convertRelativeAttribute(rawAttributes[name], currentAbility, goalkeeper);
  for (const name of GOALKEEPING) result[name] = convertRelativeAttribute(rawAttributes[name], currentAbility, !goalkeeper);
  for (const name of ALWAYS_HIGH) result[name] = convertRelativeAttribute(rawAttributes[name], currentAbility);
  // Zero is retained for absent positions/uninitialized direct values. These fields
  // are not relative and must not be forced to 1, changing their meaning.
  return result;
}
