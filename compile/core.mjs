// - void is assumed to not be null
// - null is assumed to not be undefined
// as per https://www.typescriptlang.org/tsconfig/#strictNullChecks

/** @typedef {number} P32 */
/** @typedef {number} U32 */
/** @typedef {number} I32 */
/** @typedef {number} F32 */
/** @typedef {string} Char
 * Assumed to contain exactly one codepoint
 */
/** @typedef {string} Str
 * Assumed to contain at least one codepoint and at most U32$MAX bytes
 */
/** @template $In, $Out @typedef {(_: $In) => $Out} Fn */
/** @template $Yes @typedef {{ yes: $Yes } | { no: void }} Opt */
/** @typedef {{ less: void } | { equal: void } | { greater: void }} Order */
/** @template $Origin, $Part @typedef {{} & { readonly origin?: $Origin, readonly part?: $Part }} Origin */
/** @typedef {{ erased: never }} Erased */
/** @template $Origin, $ValueErased @typedef {$ValueErased & { readonly origin_isolated?: $Origin }} Origin_isolated */
/** @template $ValueErased @typedef {$ValueErased & { readonly origin_erased?: void }} Origin_erased */
/** @template $Origin @typedef {{} & { readonly uneraser_origin?: $Origin }} Origin_uneraser */
/** @template $Origin, $Item @typedef {($Item | null)[] & { readonly origin?: $Origin }} Buf */
/** @template $Part, $Item @typedef {Buf<Origin<Erased, $Part>, $Item> & { readonly origin_erased?: void }} Buf_origin_erased */
/** @template $Item @typedef {($Item | null)[]} Unset_slice
 * Assumed to contain an empty array (with spare capacity)
 */
/** @template $Origin @typedef {U32 & { readonly origin?: $Origin }} Slot */
/** @template $Origin @typedef {{ start: U32, length: U32 } & { readonly origin?: $Origin }} Span */
/** @template $Item, _$Record @typedef {[$Item, ...$Item[]]} Array */

const I32$MIN = -2147483648;
const I32$MAX = 2147483647;
const I32$MAX_DIGITS = 11;
const U32$MAX = 4294967296;
const U32$MAX_DIGITS = 11;
const F32$MAX = 3.40282347e38;
const F32$MIN = -F32$MAX;
const F32$MAX_DIGITS = 55;
/** @param {F32} n @returns {string} */
export function f32$to_string(n) {
  // This is piles of hacks. If you know something better, please tell me!

  // digit handled separately because BigInt.toString does not emit -0
  let f32_string = n < 0 ? "-" : "";
  // all functions on Number use exponent notation for >10^21
  f32_string += BigInt(Math.trunc(Math.abs(n))).toString();
  if (!Number.isInteger(n)) {
    // calculate fraction such that there are at least F32_MAX_DIGITS.
    // Technically toFixed doesn't promise support for >20 digits
    // but all environments I've tested do actually support this up to 100
    f32_string += n.toFixed(F32$MAX_DIGITS - f32_string.length + 1).slice(1); // drop the 0 in 0.
    while (f32_string.endsWith("0")) {
      f32_string = f32_string[-1];
    }
  }
  return f32_string;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin>} span
 * @returns {$Item[]}
 */
export function buf$span_slice(buf, span) {
  return /** @type $Item[] */ (buf.slice(span.start, span.start + span.length));
}

/** @param {P32} _ @returns {void} */
export function p32_rid(_) {}
/** @param {P32} p @returns {{ a: P32, b: P32, }} */
export function p32_dup(p) {
  return { a: p, b: p };
}
/** @param {P32} p @returns {U32} */
export function p32_to_u32(p) {
  return p;
}
/** @param {{ p: P32, u: U32, }} mul @returns {P32} */
export function p32_mul_clamp(mul) {
  return Math.min(U32$MAX, mul.p * mul.u);
}
/** @param {{ left: P32, right: P32, }} sides @returns {Order} */
export function p32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {{ p: P32; u: U32 }} add @returns {P32} */
export function p32_add_clamp(add) {
  return Math.min(U32$MAX, add.p + add.u);
}
/** @template $Origin @param {P32} n @returns {Origin_isolated<$Origin, P32>} */
export function p32_origin_isolate(n) {
  return n;
}
/** @param {U32} _ @returns {void} */
export function u32_rid(_) {}
/** @param {U32} u @returns {{ a: U32, b: U32, }} */
export function u32_dup(u) {
  return { a: u, b: u };
}
/** @param {U32} i @returns {Opt<P32>} */
export function u32_to_p32(i) {
  return i === 0 ? { no: undefined } : { yes: i };
}
/** @param {U32} u @returns {F32} */
export function u32_round_to_nearest_f32_else_even(u) {
  return Math.fround(u);
}
/** @param {U32} u @returns {P32} */
export function u32_successor_clamp(u) {
  return Math.min(U32$MAX, u + 1);
}
/** @param {{ a: U32; b: U32 }} add @returns {U32} */
export function u32_add_clamp(add) {
  return Math.min(U32$MAX, add.a + add.b);
}
/** @param {{ u: U32, i: I32, }} add @returns {U32} */
export function u32_add_i32_clamp(add) {
  return Math.max(0, Math.min(U32$MAX, add.u + add.i));
}
/** @param {{ a: U32, b: U32, }} mul @returns {U32} */
export function u32_mul_clamp(mul) {
  // Math.imul would overflow
  return Math.min(U32$MAX, mul.a * mul.b);
}
/** @param {{ base: U32, exponent: P32, }} pow @returns {U32} */
export function u32_pow_clamp(pow) {
  return Math.min(U32$MAX, Math.pow(pow.base, pow.exponent));
}
/** @param {{ left: U32, right: U32, }} sides @returns {Order} */
export function u32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {U32} u @returns {P32} */
export function u32_to_i32_clamp(u) {
  return Math.min(I32$MAX, u);
}
/** @template $Origin @param {U32} n @returns {Origin_isolated<$Origin, U32>} */
export function u32_origin_isolate(n) {
  return n;
}

/** @param {I32} _ @returns {void} */
export function i32_rid(_) {}
/** @param {I32} n @returns {{ a: I32, b: I32, }} */
export function i32_dup(n) {
  return { a: n, b: n };
}
/** @param {I32} i @returns {Opt<U32>} */
export function i32_to_u32(i) {
  return i < 0 ? { no: undefined } : { yes: i };
}
/** @param {I32} i @returns {F32} */
export function i32_round_to_nearest_f32_else_even(i) {
  return Math.fround(i);
}
/** @param {I32} i @returns {U32} */
export function i32_abs_to_u32(i) {
  return Math.abs(i);
}
/** @param {I32} i @returns {U32} */
export function i32_negate_clamp(i) {
  return Math.max(I32$MIN, -i);
}
/** @param {{ left: I32, right: I32, }} sides @returns {Order} */
export function i32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {{ a: I32; b: I32 }} add @returns {I32} */
export function i32_add_clamp(add) {
  return Math.max(I32$MIN, Math.min(I32$MAX, add.a + add.b));
}
/** @param {{ a: I32, b: I32, }} mul @returns {I32} */
export function i32_mul_clamp(mul) {
  return Math.max(I32$MIN, Math.min(I32$MAX, mul.a * mul.b));
}
/** @param {{ base: I32, exponent: P32, }} power @returns {I32} */
export function i32_pow_clamp(power) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.pow(power.base, power.exponent)));
}
/** @template $Origin @param {I32} n @returns {Origin_isolated<$Origin, I32>} */
export function i32_origin_isolate(n) {
  return n;
}
/** @param {F32} _ @returns {void} */
export function f32_rid(_) {}
/** @param {F32} n @returns {{ a: F32, b: F32, }} */
export function f32_dup(n) {
  return { a: n, b: n };
}
/** @param {void} _ @returns {F32} */
export function f32_pi(_) {
  return Math.PI;
}
/** @param {{ a: F32; b: F32 }} add @returns {F32} */
export function f32_add_clamp(add) {
  return add.a + add.b;
}
/** @param {{ a: F32, b: F32, }} mul @returns {F32} */
export function f32_mul_clamp(mul) {
  return Math.max(Number.MIN_VALUE, Math.min(Number.MAX_VALUE, mul.a * mul.b));
}
/** @param {{ n: F32, by: F32, }} div @returns {F32} */
export function f32_div_clamp(div) {
  return div.by === 0
    ? 0
    : Math.max(F32$MIN, Math.min(F32$MAX, Math.fround(div.n / div.by)));
}
/** @param {F32} n @returns {Opt<F32>} */
export function f32_square_root(n) {
  return n < 0.0 ? { no: undefined } : { yes: Math.sqrt(n) };
}
/** @param {{ base: F32, exponent: I32, }} pow @returns {Opt<F32>} */
export function f32_pow_i32(pow) {
  return f32_pow(pow);
}
/** @param {{ base: F32, exponent: F32, }} pow @returns {Opt<F32>} */
export function f32_pow(pow) {
  const power = Math.pow(pow.base, pow.exponent);
  return Number.isFinite(power) && power >= F32$MIN && power <= F32$MAX
    ? { yes: Math.fround(power) }
    : { no: undefined };
}
/** @param {F32} n @returns {F32} */
export function f32_exp(n) {
  return Math.min(Number.MAX_VALUE, Math.exp(n));
}
/** @param {F32} n @returns {Opt<F32>} */
export function f32_ln(n) {
  if (n <= 0) {
    return { no: undefined };
  } else {
    const ln_result = Math.log(n);
    return Number.isFinite(ln_result)
      ? { yes: Math.fround(ln_result) }
      : { no: undefined };
  }
}
/** @param {F32} n @returns {F32} */
export function f32_sin(n) {
  return Math.fround(Math.sin(n));
}
/** @param {F32} n @returns {F32} */
export function f32_tan(n) {
  return Math.fround(Math.tan(n));
}
/** @param {F32} n @returns {F32} */
export function f32_cos(n) {
  return Math.fround(Math.cos(n));
}
/** @param {F32} n @returns {F32} */
export function f32_atan(n) {
  return Math.fround(Math.atan(n));
}
/** @param {F32} n @returns {F32} */
export function f32_abs(n) {
  return Math.abs(n);
}
/** @param {F32} n @returns {F32} */
export function f32_negate(n) {
  return -n;
}
/** @param {F32} n @returns {F32} */
export function f32_round_down(n) {
  return Math.floor(n);
}
/** @param {F32} n @returns {F32} */
export function f32_round_nearest_else_even(n) {
  const int_part_is_even = Math.trunc(n) % 2 === 0;
  return n < 0 === int_part_is_even
    ? // negative even or positive odd, e.g.
      // -12.5 -> -12
      // 1.5 -> 2
      Math.round(n)
    : // positive even or negative odd, e.g.
      // -1.5 -> -2
      // 12.5 -> 12
      -Math.round(-n);
}
/** @param {F32} n @returns {F32} */
export function f32_round_up(n) {
  return Math.ceil(n);
}
/** @param {F32} n @returns {I32} */
export function f32_round_up_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_up(n)));
}
/** @param {F32} n @returns {I32} */
export function f32_round_nearest_else_even_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_nearest_else_even(n)));
}
/** @param {F32} n @returns {F32} */
export function f32_round_nearest_else_away_from_0(n) {
  return Math.sign(n) * Math.round(Math.abs(n));
}
/** @param {F32} n @returns {I32} */
export function f32_round_nearest_else_away_from_0_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_nearest_else_away_from_0(n)));
}
/** @param {F32} n @returns {F32} */
export function f32_round_away_from_0(n) {
  return n < 0 ? Math.floor(n) : Math.ceil(n);
}
/** @param {F32} n @returns {I32} */
export function f32_round_away_from_0_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_away_from_0(n)));
}
/** @param {F32} n @returns {I32} */
export function f32_round_down_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.floor(n)));
}
/** @param {F32} n @returns {F32} */
export function f32_round_toward_0(n) {
  return Math.trunc(n);
}
/** @param {F32} n @returns {I32} */
export function f32_round_toward_0_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.trunc(n)));
}
/** @param {{ left: F32, right: F32, }} sides @returns {Order} */
export function f32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @template $Origin @param {F32} n @returns {Origin_isolated<$Origin, F32>} */
export function f32_origin_isolate(n) {
  return n;
}

/** @param {Char} _ @returns {void} */
export function char_rid(_) {}
/** @param {Char} c @returns {{ a: Char, b: Char, }} */
export function char_dup(c) {
  return { a: c, b: c };
}
/** @param {Char} char @returns {U32} */
export function char_to_u32(char) {
  return char.charCodeAt(0);
}
/** @template $Origin @param {Char} c @returns {Origin_isolated<$Origin, Char>} */
export function char_origin_isolate(c) {
  return c;
}

/** @param {Str} _ @returns {void} */
export function str_rid(_) {}
/** @param {Str} str @returns {{ a: Str, b: Str, }} */
export function str_dup(str) {
  return { a: str, b: str };
}
/** @param {Str} str @returns {P32} */
export function str_char_count(str) {
  let count = 0;
  for (const _ of str) {
    count++;
  }
  return count;
}
/** @param {Str} str @returns {P32} */
export function str_utf8_length(str) {
  // probably wasteful but couldn't find something native other than this
  // new TextEncoder().encode(str).length
  return new Blob([str]).size;
}
/** @param {Str} str @returns {{ start: Char, after: Opt<Str>, }} */
export function str_start(str) {
  let first = str.charAt(0);
  return {
    start: first,
    after:
      str.length === first.length ? { no: undefined } : { yes: str.slice(first.length) },
  };
}
/** @param {Str} str @returns {{ end: Char, before: Opt<Str>, }} */
export function str_end(str) {
  const lastOneOrTwo = str.slice(-2);
  return lastOneOrTwo.charAt(0).length === 2
    ? // last codepoint consists of 2 code units
      {
        end: lastOneOrTwo,
        before: str.length === 2 ? { no: undefined } : { yes: str.slice(0, -2) },
      }
    : // last codepoint consists of 1 code unit
      {
        end: str.slice(-1),
        before: str.length === 1 ? { no: undefined } : { yes: str.slice(0, -1) },
      };
}
/** @template $Origin @param {Str} s @returns {Origin_isolated<$Origin, Str>} */
export function str_origin_isolate(s) {
  return s;
}

/** @template $Yes @param {$Yes} yes @returns {Opt<$Yes>} */
export function opt_yes(yes) {
  return { yes: yes };
}

/** @template $Result @param {never} never @returns {$Result} */
export function choice_empty_to(never) {
  return never;
}

/** @template $In, $Out @param {Fn<$In, $Out>} _ @returns {void} */
export function fn_rid(_) {}
/** @template $In, $Out @param {Fn<$In, $Out>} f @returns {{ a: Fn<$In, $Out>, b: Fn<$In, $Out>, }} */
export function fn_dup(f) {
  return { a: f, b: f };
}
/** @template $In, $Out @param {{ fn: Fn<$In, $Out>, inø: $In, }} call @returns {$Out} */
export function call(call) {
  return call.fn(call.inø);
}
/** @template $In, $Origin, $Out @param {Fn<$In, $Out>} f @returns {Origin_isolated<$Origin, Fn<$In, $Out>>} */
export function fn_origin_isolate(f) {
  return f;
}

/** @template $Origin, $Part @param {Origin<$Origin, $Part>} _ @returns {void} */
export function origin_rid(_) {}

/** @template $Origin, $Value_erased @param {Fn<void, $Value_erased>} erase @returns {Origin_isolated<$Origin, $Value_erased>} */
export function origin_isolate_constant(erase) {
  return /** @type Origin_isolated<$Origin, $Value_erased> */ (erase());
}
/** @template $Origin, $A, $B @param {{ isolated: Origin_isolated<$Origin, $A>, change: Fn<$A, $B>, }} map @returns {Origin_isolated<$Origin, $B>} */
export function origin_isolated_map(map) {
  return /** @type Origin_isolated<$Origin, $B> */ (map.change(map.isolated));
}
/** @template $Origin, $A, $B @param {{ a: Origin_isolated<$Origin, $A>, b: Origin_isolated<$Origin, $B>, }} ab @returns {Origin_isolated<$Origin, { a: $A, b: $B }>} */
export function origin_isolated_merge(ab) {
  return ab;
}
/** @template $Origin, $Value_erased @param {Origin_isolated<$Origin, $Value_erased>} erase @returns {Origin_erased<$Value_erased>} */
export function origin_erase(erase) {
  return erase;
}
/** @template $Value_erased @param {{ erased: Origin_erased<$Value_erased>, rid: Fn<$Value_erased, void>, }} _ @returns {void} */
export function origin_erased_rid(_) {}
/** @template $Origin, $Value, $Value_erased @param {{ erased: Origin_erased<$Value_erased>, origin: Origin<$Origin, void>, unerase: Fn<{ erased: $Value_erased, uneraser: Origin_uneraser<$Origin>, }, { unerased: $Value, uneraser: Origin_uneraser<$Origin>, }>, }} unerase @returns {$Value} */
export function origin_unerase(unerase) {
  return unerase.unerase({ erased: unerase.erased, uneraser: {} }).unerased;
}

/** @template $Origin @param {Slot<$Origin>} slot @returns {{ slot: Slot<$Origin>, index: U32 }} */
export function slot_index(slot) {
  return { slot: slot, index: slot };
}
/** @template $Origin @param {Slot<$Origin>} slot @returns {Span<$Origin>} */
export function slot_to_span(slot) {
  return { start: slot, length: 1 };
}
/** @template $Origin, $Part @param {Slot<Origin<$Origin, $Part>>} slot @returns {Origin_isolated<$Origin, Slot<Origin<Erased, $Part>>>} */
export function slot_origin_isolate(slot) {
  return /** @type Origin_isolated<$Origin, Slot<Origin<Erased, $Part>>> */ (slot);
}
/** @template $Origin, $Part @param {{ slot: Slot<Origin<Erased, $Part>>, uneraser: Origin_uneraser<$Origin>, }} unerase @returns {{ slot: Slot<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin>, }} */
export function slot_origin_unerase(unerase) {
  return /** @type {{ slot: Slot<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin> }} */ (
    unerase
  );
}

/** @template $Origin, $Part @param {Span<Origin<$Origin, $Part>>} span @returns {Origin_isolated<$Origin, Span<Origin<Erased, $Part>>>} */
export function span_origin_isolate(span) {
  return /** @type Origin_isolated<$Origin, Span<Origin<Erased, $Part>>> */ (span);
}
/** @template $Origin, $Part @param {Opt<Span<Origin<$Origin, $Part>>>} span @returns {Opt<Span<Origin<Erased, $Part>>>} */
export function opt_span_origin_isolate(span) {
  return /** @type Opt<Span<Origin<Erased, $Part>>> */ (span);
}
/** @template $Origin, $Part @param {{ span: Span<Origin<Erased, $Part>>, uneraser: Origin_uneraser<$Origin>, }} span @returns {{ span: Span<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin>, }} */
export function span_origin_unerase(span) {
  return /** @type {{ span: Span<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin> }} */ (
    span
  );
}
/** @template $Origin, $Part @param {{ span: Opt<Span<Origin<Erased, $Part>>>, uneraser: Origin_uneraser<$Origin>, }} span @returns {{ span: Opt<Span<Origin<$Origin, $Part>>>, uneraser: Origin_uneraser<$Origin>, }} */
export function opt_span_origin_unerase(span) {
  return /** @type {{ span: Opt<Span<Origin<$Origin, $Part>>>, uneraser: Origin_uneraser<$Origin> }} */ (
    span
  );
}
/** @template $Origin @param {Span<$Origin>} span @returns {{ span: Span<$Origin>, index: U32, }} */
export function span_start_index(span) {
  return { span: span, index: span.start };
}
/** @template $Origin @param {Span<$Origin>} span @returns {{ span: Span<$Origin>, length: P32, }} */
export function span_length(span) {
  return { span: span, length: span.length };
}
/** @template $Origin @param {Opt<Span<$Origin>>} span @returns {{ span: Opt<Span<$Origin>>, length: U32, }} */
export function opt_span_length(span) {
  return { span: span, length: "no" in span ? 0 : span.yes.length };
}
/** @template $Origin @param {Span<$Origin>} span @returns {{ start: Slot<$Origin>, after: Opt<Span<$Origin>>, }} */
export function span_start(span) {
  return {
    start: span.start,
    after:
      span.length >= 2
        ? { yes: { start: span.start + 1, length: span.length - 1 } }
        : { no: undefined },
  };
}
/** @template $Origin @param {Span<$Origin>} span @returns {{ end: Slot<$Origin>, before: Opt<Span<$Origin>>, }} */
export function span_end(span) {
  return {
    end: span.start + span.length - 1,
    before:
      span.length >= 2
        ? { yes: { start: span.start, length: span.length - 1 } }
        : { no: undefined },
  };
}
/** @template $Origin @param {{ span: Span<$Origin>, length: P32, }} take @returns {{ start: Span<$Origin>, after: Opt<Span<$Origin>>, }} */
export function span_start_of_length_positive(take) {
  return {
    start: { start: take.span.start, length: Math.min(take.span.length, take.length) },
    after:
      take.span.length <= take.length
        ? { no: undefined }
        : {
            yes: {
              start: take.span.start + take.length,
              length: take.span.length - take.length,
            },
          },
  };
}
/** @template $Origin @param {{ span: Span<$Origin>, length: P32, }} take @returns {{ end: Span<$Origin>, before: Opt<Span<$Origin>>, }} */
export function span_end_of_length_positive(take) {
  return {
    end: {
      start: take.span.start + Math.max(0, take.span.length - take.length),
      length: take.length,
    },
    before:
      take.span.length <= take.length
        ? { no: undefined }
        : {
            yes: {
              start: take.span.start,
              length: take.span.length - take.length,
            },
          },
  };
}
/** @template $Origin, $State
 * @param {{
 *     span: Span<$Origin>,
 *     direction: { up: void } | { down: void },
 *     state: $State,
 *     step: Fn<{ slot: Slot<$Origin>, state: $State, }, $State>,
 * }} step
 * @returns {$State} */
export function span_step(step) {
  let state = step.state;
  if ("up" in step.direction) {
    for (let i = step.span.start; i < step.span.start + step.span.length; i++) {
      state = step.step({ state: state, slot: i });
    }
  } else {
    for (let i = step.span.start + step.span.length - 1; i >= step.span.start; i--) {
      state = step.step({ state: state, slot: i });
    }
  }
  return state;
}
/** @template $Origin, $State
 * @param {{
 *     span: Opt<Span<$Origin>>,
 *     direction: { up: void } | { down: void },
 *     state: $State,
 *     step: Fn<{ slot: Slot<$Origin>, state: $State, }, $State>,
 * }} step
 * @returns {$State} */
export function opt_span_step(step) {
  if ("no" in step.span) return step.state;
  return span_step({
    span: step.span.yes,
    direction: step.direction,
    state: step.state,
    step: step.step,
  });
}
/** @template $Done, $Going @param {$Done} done @returns {{ going: $Going } | { done: $Done }} */
export function done(done) {
  return { done: done };
}
/** @template $Done, $Going @param {$Going} going @returns {{ done: $Done } | { going: $Going }} */
export function going(going) {
  return { going: going };
}
/** @template $Done, $Going, $Origin
 * @param {{
 *     span: Span<$Origin>,
 *     direction: { up: void } | { down: void },
 *     state: $Going,
 *     step: Fn<{ slot: Slot<$Origin>, state: $Going, }, { going: $Going } | { done: $Done }>,
 * }} step
 * @returns {{ going: $Going } | { done: { done: $Done, rest: Opt<Span<$Origin>>} }} */
export function span_step_while(step) {
  let state = step.state;
  if ("up" in step.direction) {
    for (let i = step.span.start; i < step.span.start + step.span.length; i++) {
      const step_result = step.step({ state: state, slot: i });
      if ("done" in step_result) {
        const rest_length = step.span.start + step.span.length - 1 - i;
        return {
          done: {
            done: step_result.done,
            rest:
              rest_length >= 1
                ? { yes: { start: i + 1, length: rest_length } }
                : { no: undefined },
          },
        };
      }
      state = step_result.going;
    }
  } else {
    for (let i = step.span.start + step.span.length - 1; i >= step.span.start; i--) {
      const step_result = step.step({ state: state, slot: i });
      if ("done" in step_result) {
        const rest_length = i - step.span.start;
        return {
          done: {
            done: step_result.done,
            rest:
              rest_length >= 1
                ? { yes: { start: 0, length: rest_length } }
                : { no: undefined },
          },
        };
      }
      state = step_result.going;
    }
  }
  return { going: state };
}
/** @template $Done, $Going, $Origin
 * @param {{
 *     span: Opt<Span<$Origin>>,
 *     direction: { up: void } | { down: void },
 *     state: $Going,
 *     step: Fn<{ slot: Slot<$Origin>, state: $Going, }, { going: $Going } | { done: $Done }>,
 * }} step
 * @returns {{ going: $Going } | { done: { done: $Done, rest: Opt<Span<$Origin>>} }} */
export function opt_span_step_while(step) {
  if ("no" in step.span) return { going: step.state };
  return span_step_while({
    span: step.span.yes,
    direction: step.direction,
    state: step.state,
    step: step.step,
  });
}

/** @template $Item, $Origin, $Part @param {Origin<$Origin, $Part>} _ @returns {Buf<$Origin, $Item>} */
export function buf_empty(_) {
  return [];
}
/** @template $Item, $Origin @param {Buf<$Origin, $Item>} _ @returns {void} */
export function buf_rid(_) {}
/** @template $Item, $Origin @param {Buf<$Origin, $Item>} buf @returns {Buf<$Origin, $Item>} */
export function buf_pre_allocation_rid(buf) {
  return buf;
}
/** @template $Item, $Item_erased, $Origin, $Part @param {{ buf: Buf<Origin<$Origin, $Part>, $Item>, item_isolate: Fn<$Item, Origin_isolated<$Origin, $Item_erased>>, }} erase @returns {Buf_origin_erased<$Part, $Item_erased>} */
export function buf_origin_isolate(erase) {
  return erase.buf.map((item) => (item === null ? null : erase.item_isolate(item)));
}
/** @template $Item, $Origin, $Part @param {{ buf: Buf_origin_erased<$Part, $Item>, uneraser: Origin_uneraser<$Origin>, }} unerase @returns {{ buf: Buf<Origin<$Origin, $Part>, $Item>, uneraser: Origin_uneraser<$Origin>, }} */
export function buf_origin_unerase_keep_items(unerase) {
  return {
    buf: /** @type Buf<Origin<$Origin, $Part>, $Item> */ (unerase.buf),
    uneraser: unerase.uneraser,
  };
}
/** @template $Item, $Item_erased, $Origin, $Part @param {{ buf: Buf_origin_erased<$Part, $Item_erased>, uneraser: Origin_uneraser<$Origin>, item_unerase: Fn<{ item: $Item_erased, uneraser: Origin_uneraser<$Origin>, }, { item: $Item, uneraser: Origin_uneraser<$Origin>, }>, }} unerase @returns {{ buf: Buf<Origin<$Origin, $Part>, $Item>, uneraser: Origin_uneraser<$Origin>, }} */
export function buf_origin_unerase(unerase) {
  return {
    buf: unerase.buf.map((item) =>
      item === null
        ? null
        : unerase.item_unerase({
            item: item,
            uneraser: unerase.uneraser,
          }).item,
    ),
    uneraser: unerase.uneraser,
  };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, item_rid: Fn<$Item, void>, span: Span<$Origin>, }} unset @returns {Buf<$Origin, $Item>} */
export function buf_span_rid(unset) {
  if (unset.span.start + unset.span.length === unset.buf.length) {
    for (let i = unset.span.start; i < unset.span.start + unset.span.length; i++) {
      unset.item_rid(/** @type $Item */ (unset.buf[i]));
    }
    unset.buf.length -= unset.span.length;
    while (unset.buf[unset.buf.length - 1] === null) {
      unset.buf.length -= 1;
    }
  } else {
    for (let i = unset.span.start; i < unset.span.start + unset.span.length; i++) {
      unset.item_rid(/** @type $Item */ (unset.buf[i]));
      unset.buf[i] = null;
    }
  }
  return unset.buf;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, item_rid: Fn<$Item, void>, span: Opt<Span<$Origin>>, }} unset @returns {Buf<$Origin, $Item>} */
export function buf_opt_span_rid(unset) {
  if ("yes" in unset.span) {
    for (
      let i = unset.span.yes.start;
      i < unset.span.yes.start + unset.span.yes.length;
      i++
    ) {
      unset.item_rid(/** @type $Item */ (unset.buf[i]));
      unset.buf[i] = null;
    }
  }
  return unset.buf;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, length: U32, }} pre_allocate @returns {Buf<$Origin, $Item>} */
export function buf_pre_allocate_at_least(pre_allocate) {
  pre_allocate.buf.length += pre_allocate.length;
  pre_allocate.buf.length -= pre_allocate.length;
  return pre_allocate.buf;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, newø: $Item, }} add @returns {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, }} */
export function buf_add(add) {
  const new_index = add.buf.length;
  add.buf.push(add.newø);
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return { buf: add.buf, slot: new_index };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, newø: $Item, }} insert @returns {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, }} */
export function buf_insert(insert) {
  const existing_unset_index = insert.buf.findIndex((el) => el === null);
  if (existing_unset_index >= 0) {
    insert.buf[existing_unset_index] = insert.newø;
    return { buf: insert.buf, slot: existing_unset_index };
  } else {
    return buf_add(insert);
  }
}
/** @template $Item, $Origin @template $Record @param {{ buf: Buf<$Origin, $Item>, newø: Array<$Item, $Record>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_add_array(add) {
  add.buf.push(...add.newø);
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: { start: add.buf.length - add.newø.length, length: add.newø.length },
  };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, newø: $Item, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_span_add(add) {
  if (add.span.start + add.span.length < add.buf.length) {
    // move span to end
    for (let i = add.span.start; i < add.span.start + add.span.length; i++) {
      add.buf.push(add.buf[i]);
      add.buf[i] = null;
    }
  }
  add.buf.push(add.newø);
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: { start: add.buf.length - 1 - add.span.length, length: add.span.length + 1 },
  };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, newø: $Item, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_opt_span_add(add) {
  if ("no" in add.span) {
    const new_index = add.buf.length;
    add.buf.push(add.newø);
    if (add.buf.length > U32$MAX)
      throw Error("Array length " + add.buf.length + " not representable as a u32");
    return { buf: add.buf, span: slot_to_span(new_index) };
  }
  return buf_span_add({ buf: add.buf, span: add.span.yes, newø: add.newø });
}
/** @template $Item, $Origin @template $Record @param {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, newø: Array<$Item, $Record>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_span_add_array(add) {
  if (add.span.start + add.span.length < add.buf.length) {
    // move span to end
    for (let i = add.span.start; i < add.span.start + add.span.length; i++) {
      add.buf.push(add.buf[i]);
      add.buf[i] = null;
    }
  }
  add.buf.push(...add.newø);
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: {
      start: add.buf.length - add.newø.length - add.span.length,
      length: add.span.length + add.newø.length,
    },
  };
}
/** @template $Item, $Origin @template $Record @param {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, newø: Array<$Item, $Record>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_opt_span_add_array(add) {
  if ("no" in add.span) {
    add.buf.push(...add.newø);
    if (add.buf.length > U32$MAX)
      throw Error("Array length " + add.buf.length + " not representable as a u32");
    return {
      buf: add.buf,
      span: {
        start: add.buf.length - add.newø.length,
        length: add.newø.length,
      },
    };
  }
  return buf_span_add_array({ buf: add.buf, span: add.span.yes, newø: add.newø });
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, }} remove @returns {{ buf: Buf<$Origin, $Item>, item: $Item, }} */
export function buf_remove(remove) {
  if (remove.slot + 1 < remove.buf.length) {
    const item = /** @type {$Item} */ (remove.buf[remove.slot]);
    remove.buf[remove.slot] = null;
    return { buf: remove.buf, item: item };
  } else {
    const item = /** @type {$Item} */ (remove.buf.pop());
    while (remove.buf[remove.buf.length - 1] === null) {
      remove.buf.length -= 1;
    }
    return { buf: remove.buf, item: item };
  }
}
/** @template $Item, $Origin
 * @param {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, new: $Item, }} replace
 * @returns {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, item: $Item,}} */
export function buf_replace(replace) {
  const old_item = /** @type {$Item} */ (replace.buf[replace.slot]);
  replace.buf[replace.slot] = replace.new;
  return { buf: replace.buf, slot: replace.slot, item: old_item };
}
/** @template $In, $Item, $Origin, $Out
 * @param {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, in: $In, step: Fn<{in:$In, item:$Item}, {item: $Item, out:$Out}>,}} step
 * @returns {{ buf: Buf<$Origin, $Item>, slot: Slot<$Origin>, out: $Out }} */
export function buf_item_step(step) {
  const item = /** @type {$Item} */ (step.buf[step.slot]);
  const stepped = step.step({ in: step.in, item: item });
  step.buf[step.slot] = stepped.item;
  return { buf: step.buf, slot: step.slot, out: stepped.out };
}
/** @template $Item, $Origin
 * @param {{ buf: Buf<$Origin, $Item>, slot_a: Slot<$Origin>, slot_b: Slot<$Origin>,}} swap
 * @returns {{ buf: Buf<$Origin, $Item>, slot_a: Slot<$Origin>, slot_b: Slot<$Origin>,}} */
export function buf_swap(swap) {
  const a_item = swap.buf[swap.slot_a];
  swap.buf[swap.slot_a] = swap.buf[swap.slot_b];
  swap.buf[swap.slot_b] = a_item;
  return { buf: swap.buf, slot_a: swap.slot_b, slot_b: swap.slot_a };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} move @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_span_move_to_end(move) {
  if (move.span.start + move.span.length < move.buf.length) {
    // move span to end
    for (let i = move.span.start; i < move.span.start + move.span.length; i++) {
      move.buf.push(move.buf[i]);
      move.buf[i] = null;
    }
    if (move.buf.length > U32$MAX)
      throw Error("Array length " + move.buf.length + " not representable as a u32");
  }
  return {
    buf: move.buf,
    span: { start: move.buf.length - move.span.length, length: move.span.length },
  };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} move @returns {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} */
export function buf_opt_span_move_to_end(move) {
  if ("no" in move.span) return { buf: move.buf, span: { no: undefined } };
  const span = move.span.yes;
  if (span.start + span.length < move.buf.length) {
    // move span to end
    for (let i = span.start; i < span.start + span.length; i++) {
      move.buf.push(move.buf[i]);
      move.buf[i] = null;
    }
    if (move.buf.length > U32$MAX)
      throw Error("Array length " + move.buf.length + " not representable as a u32");
  }
  return {
    buf: move.buf,
    span: { yes: { start: move.buf.length - span.length, length: span.length } },
  };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} move @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_span_move_to_unset(move) {
  if (move.span.start + move.span.length < move.buf.length) return move;
  let unset_length = 0;
  for (let i = 0; i < move.buf.length; i++) {
    if (move.buf[i] === null) {
      unset_length++;
      if (unset_length === move.span.length) {
        const unset_start = i - move.span.length + 1;
        for (let unset_i = 0; unset_i < move.span.length; unset_i++) {
          move.buf[unset_start + unset_i] = move.buf[move.span.start + unset_i];
        }
        move.buf.length -= move.span.length;
        while (move.buf[move.buf.length - 1] === null) {
          move.buf.length -= 1;
        }
        return { buf: move.buf, span: { start: unset_start, length: move.span.length } };
      }
    } else {
      unset_length = 0;
    }
  }
  return move;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} move @returns {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} */
export function buf_opt_span_move_to_unset(move) {
  if ("no" in move.span) return move;
  const span = move.span.yes;
  if (span.start + span.length < move.buf.length) return move;
  let unset_length = 0;
  for (let i = 0; i < move.buf.length; i++) {
    if (move.buf[i] === null) {
      unset_length++;
      if (unset_length === span.length) {
        const unset_start = i - span.length + 1;
        for (let unset_i = 0; unset_i < span.length; unset_i++) {
          move.buf[unset_start + unset_i] = move.buf[span.start + unset_i];
        }
        move.buf.length -= span.length;
        while (move.buf[move.buf.length - 1] === null) {
          move.buf.length -= 1;
        }
        return {
          buf: move.buf,
          span: { yes: { start: unset_start, length: span.length } },
        };
      }
    } else {
      unset_length = 0;
    }
  }
  return move;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} reverse @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_span_reverse(reverse) {
  const slice = reverse.buf.slice(
    reverse.span.start,
    reverse.span.start + reverse.span.length,
  );
  slice.reverse();
  reverse.buf.splice(reverse.span.start, reverse.span.length, ...slice);
  return reverse;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} reverse @returns {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} */
export function buf_opt_span_reverse(reverse) {
  if ("no" in reverse.span) return reverse;
  const span = reverse.span.yes;
  const slice = reverse.buf.slice(span.start, span.start + span.length);
  slice.reverse();
  reverse.buf.splice(span.start, span.length, ...slice);
  return reverse;
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, start: Span<$Origin>, end: Span<$Origin>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }}} */
export function buf_span_add_own_span(add) {
  if (add.start.start + add.start.length === add.end.start) {
    return {
      buf: add.buf,
      span: { start: add.start.start, length: add.start.length + add.end.length },
    };
  }
  if (add.start.start + add.start.length < add.buf.length) {
    // move start span to end
    for (let i = add.start.start; i < add.start.start + add.start.length; i++) {
      add.buf.push(add.buf[i]);
      add.buf[i] = null;
    }
  }
  // move end span to end after the start items
  for (let i = add.end.start; i < add.end.start + add.end.length; i++) {
    add.buf.push(add.buf[i]);
    add.buf[i] = null;
  }
  return {
    buf: add.buf,
    span: {
      start: add.buf.length - add.start.length - add.end.length,
      length: add.start.length + add.end.length,
    },
  };
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, start: Span<$Origin>, end: Opt<Span<$Origin>>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_span_add_own_opt_span(add) {
  if ("no" in add.end) {
    return { buf: add.buf, span: add.start };
  }
  return buf_span_add_own_span({ buf: add.buf, start: add.start, end: add.end.yes });
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, start: Opt<Span<$Origin>>, end: Span<$Origin>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Span<$Origin>, }} */
export function buf_opt_span_add_own_span(add) {
  if ("no" in add.start) {
    return { buf: add.buf, span: add.end };
  }
  return buf_span_add_own_span({ buf: add.buf, start: add.start.yes, end: add.end });
}
/** @template $Item, $Origin @param {{ buf: Buf<$Origin, $Item>, start: Opt<Span<$Origin>>, end: Opt<Span<$Origin>>, }} add @returns {{ buf: Buf<$Origin, $Item>, span: Opt<Span<$Origin>>, }} */
export function buf_opt_span_add_own_opt_span(add) {
  if ("no" in add.start) {
    return { buf: add.buf, span: add.end };
  }
  if ("no" in add.end) {
    return { buf: add.buf, span: add.start };
  }
  if (add.start.yes.start + add.start.yes.length === add.end.yes.start) {
    return {
      buf: add.buf,
      span: {
        yes: {
          start: add.start.yes.start,
          length: add.start.yes.length + add.end.yes.length,
        },
      },
    };
  }
  if (add.start.yes.start + add.start.yes.length < add.buf.length) {
    for (
      let i = add.start.yes.start;
      i < add.start.yes.start + add.start.yes.length;
      i++
    ) {
      add.buf.push(add.buf[i]);
      add.buf[i] = null;
    }
  }
  for (let i = add.end.yes.start; i < add.end.yes.start + add.end.yes.length; i++) {
    add.buf.push(add.buf[i]);
    add.buf[i] = null;
  }
  return {
    buf: add.buf,
    span: {
      yes: {
        start: add.buf.length - add.start.yes.length - add.end.yes.length,
        length: add.start.yes.length + add.end.yes.length,
      },
    },
  };
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, newø: Str, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_add_str(add) {
  const new_start = add.buf.length;
  for (const new_char of add.newø) {
    add.buf.push(new_char);
  }
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: {
      start: new_start,
      length: add.buf.length - new_start,
    },
  };
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, newø: Str, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_span_add_str(add) {
  if (add.span.start + add.span.length < add.buf.length) {
    // move span to end
    for (let i = add.span.start; i < add.span.start + add.span.length; i++) {
      add.buf.push(add.buf[i]);
      add.buf[i] = null;
    }
  }
  const new_start = add.buf.length;
  for (const new_char of add.newø) {
    add.buf.push(new_char);
  }
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: {
      start: new_start - add.span.length,
      length: add.span.length + (add.buf.length - new_start),
    },
  };
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Opt<Span<$Origin>>, newø: Str, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_opt_span_add_str(add) {
  if ("no" in add.span) {
    return buf_char_add_str(add);
  }
  return buf_char_span_add_str({ buf: add.buf, span: add.span.yes, newø: add.newø });
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, newø: U32, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_span_add_u32(add) {
  return buf_char_span_add_str({
    buf: add.buf,
    span: add.span,
    newø: add.newø.toPrecision(U32$MAX_DIGITS),
  });
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Opt<Span<$Origin>>, newø: U32, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_opt_span_add_u32(add) {
  return buf_char_opt_span_add_str({
    buf: add.buf,
    span: add.span,
    newø: add.newø.toPrecision(U32$MAX_DIGITS),
  });
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, newø: I32, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_span_add_i32(add) {
  return buf_char_span_add_str({
    buf: add.buf,
    span: add.span,
    newø: add.newø.toPrecision(I32$MAX_DIGITS),
  });
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Opt<Span<$Origin>>, newø: I32, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_opt_span_add_i32(add) {
  return buf_char_opt_span_add_str({
    buf: add.buf,
    span: add.span,
    newø: add.newø.toPrecision(I32$MAX_DIGITS),
  });
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, newø: F32, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_span_add_f32(add) {
  return buf_char_span_add_str({
    buf: add.buf,
    span: add.span,
    newø: f32$to_string(add.newø),
  });
}
/** @template $Origin @param {{ buf: Buf<$Origin, Char>, span: Opt<Span<$Origin>>, newø: F32, }} add @returns {{ buf: Buf<$Origin, Char>, span: Span<$Origin>, }} */
export function buf_char_opt_span_add_f32(add) {
  return buf_char_opt_span_add_str({
    buf: add.buf,
    span: add.span,
    newø: f32$to_string(add.newø),
  });
}
/** @template $Item, $Origin @param {Buf<$Origin, $Item>} buf @returns {Unset_slice<$Item>} */
export function buf_to_unset(buf) {
  buf.length = 0;
  return buf;
}
/** @template $Item, $Origin, $Part @param {{ origin: Origin<$Origin, $Part>, slice: Unset_slice<$Item>, }} reuse @returns {Buf<$Origin, $Item>} */
export function buf_reuse(reuse) {
  return reuse.slice;
}

/** @template $Item @param {Unset_slice<$Item>} _ @returns {void} */
export function unset_slice_rid(_) {}
/** @template $Item @param {U32} length @returns {Unset_slice<$Item>} */
export function unset_slice_allocate_length(length) {
  const array = Array(length);
  array.length = 0;
  return array;
}
/** @template $Item, $New_item @param {Unset_slice<$Item>} unset_slice @returns {Unset_slice<$New_item>} */
export function unset_slice_cast_or_rid_and_allocate(unset_slice) {
  // for once equal type sizes come in clutch
  return /** @type ($New_item | null)[] */ (unset_slice);
}
