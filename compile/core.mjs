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
 * Assumed to contain at least one codepoint
 */
/** @template $In, $Out @typedef {(_: $In) => $Out} Fn */
/** @template $Yes @typedef {{ yes: $Yes } | { no: void }} Opt */
/** @typedef {{ less: void } | { equal: void } | { greater: void }} Order */
/** @template $Origin, $Part @typedef {{} & { readonly origin?: $Origin, readonly part?: $Part }} Origin */
/** @typedef {{ erased: never }} Erased */
/** @template $Part, $Rest @typedef {{ part: $Part, rest: $Rest }} Part_rest */
/** @template $Parts, $Value @typedef {$Value & { readonly parts?: $Parts }} Origin_erased */
/** @template $Origin, $Part @typedef {{} & { readonly eraser_origin?: $Origin, readonly part?: $Part }} Origin_eraser */
/** @template $Origin, $Part @typedef {{} & { readonly uneraser_origin?: $Origin, readonly part?: $Part }} Origin_uneraser */
/** @template $Origin, $Element @typedef {($Element | null)[] & { readonly origin?: $Origin }} Buf */
/** @template $Element @typedef {($Element | null)[]} Unset_slice */
/** @template $Origin @typedef {U32 & { readonly origin?: $Origin }} Slot */
/** @template $Origin @typedef {U32 & { readonly unset_origin?: $Origin }} Unset_slot */
/** @template $Origin @typedef {{ start: U32, length: U32 } & { readonly origin?: $Origin }} Span */
/** @template $Origin @typedef {{ start: U32, length: U32 } & { readonly unset_origin?: $Origin }} Unset_span */
/** @template $Element, _$Record @typedef {[$Element, ...$Element[]]} Array */

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
/** @template $Element, $Origin
 * @param {Buf<$Origin, $Element>} buf
 * @param {Span<$Origin>} span
 * @returns {$Element[]}
 */
export function buf$span_slice(buf, span) {
  return /** @type $Element[] */ (buf.slice(span.start, span.start + span.length));
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

/** @template $Origin, $Part @param {Origin<$Origin, $Part>} _ @returns {void} */
export function origin_rid(_) {}
/** @template $Part_name, $Part_origin, $Rest_name, $Rest_origin @param {{ part: Origin<
 *
 $Part_origin, $Part_name>, rest: Origin<$Rest_origin, $Rest_name>, }} _ @returns {Origin<{ part: $Part_origin, rest: $Rest_origin, }, { part: $Part_name, rest: $Rest_name, }>} */
export function origin_add(_) {
  return {};
}
/** @template $Origin, $Part, $Rest @param {Origin<$Origin, { part: $Part, rest: $Rest, }>} _ @returns {{ part: Origin<$Origin, $Part>, rest: Origin<$Origin, $Rest>, }} */
export function origin_part(_) {
  return { part: {}, rest: {} };
}
/** @template $Origin, $Parts, $Value, $Value_erased @param {{ value: $Value, erase: Fn<{ value: $Value, eraser: Origin_eraser<$Origin, $Parts>, }, $Value_erased>, }} erase @returns {Origin_erased<$Parts, $Value_erased>} */
export function origin_erase(erase) {
  return /** @type Origin_erased<$Parts, $Value_erased>  */ (
    erase.erase({ value: erase.value, eraser: {} })
  );
}
/** @template $Origin, $Parts, $Value, $Value_erased @param {{ erased: Origin_erased<$Parts, $Value_erased>, origin: Origin<$Origin, $Parts>, unerase: Fn<{ erased: $Value_erased, uneraser: Origin_uneraser<$Origin, $Parts>, }, $Value>, value_rid: Fn<$Value, {}>, }} unerase @returns {$Value} */
export function origin_unerase(unerase) {
  return unerase.unerase({ erased: unerase.erased, uneraser: {} });
}

/** @template $Origin, $Part, $Rest @param {Origin_eraser<$Origin, { part: $Part, rest: $Rest, }>} _ @returns {{ part: Origin_eraser<$Origin, $Part>, rest: Origin_eraser<$Origin, $Rest>, }} */
export function origin_eraser_part(_) {
  return { part: {}, rest: {} };
}

/** @template $Origin, $Part, $Rest @param {Origin_uneraser<$Origin, { part: $Part, rest: $Rest, }>} _ @returns {{ part: Origin_uneraser<$Origin, $Part>, rest: Origin_uneraser<$Origin, $Rest>, }} */
export function origin_uneraser_part(_) {
  return { part: {}, rest: {} };
}

/** @template $Origin @param {Slot<$Origin>} slot @returns {Span<$Origin>} */
export function slot_to_span(slot) {
  return { start: slot, length: 1 };
}
/** @template $Origin, $Part @param {{ slot: Slot<Origin<$Origin, $Part>>, eraser: Origin_eraser<$Origin, $Part>, }} erase @returns {{ slot: Slot<Origin<Erased, $Part>>, eraser: Origin_eraser<$Origin, $Part>, }} */
export function slot_origin_erase(erase) {
  return /** @type {{ slot: Slot<Origin<Erased, $Part>>, eraser: Origin_eraser<$Origin, $Part> }} */ (
    erase
  );
}
/** @template $Origin, $Part @param {{ slot: Slot<Origin<Erased, $Part>>, uneraser: Origin_uneraser<$Origin, $Part>, }} unerase @returns {{ slot: Slot<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin, $Part>, }} */
export function slot_origin_unerase(unerase) {
  return /** @type {{ slot: Slot<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin, $Part> }} */ (
    unerase
  );
}
/** @template $Origin @param {Unset_slot<$Origin>} slot @returns {Unset_span<$Origin>} */
export function unset_slot_to_span(slot) {
  return slot_to_span(slot);
}

/** @template $Origin, $Part @param {{ span: Span<Origin<$Origin, $Part>>, eraser: Origin_eraser<$Origin, $Part>, }} span @returns {{ span: Span<Origin<Erased, $Part>>, eraser: Origin_eraser<$Origin, $Part>, }} */
export function span_origin_erase(span) {
  return /** @type {{ span: Span<Origin<Erased, $Part>>, eraser: Origin_eraser<$Origin, $Part> }} */ (
    span
  );
}
/** @template $Origin, $Part @param {{ span: Opt<Span<Origin<$Origin, $Part>>>, eraser: Origin_eraser<$Origin, $Part>, }} span @returns {{ span: Opt<Span<Origin<Erased, $Part>>>, eraser: Origin_eraser<$Origin, $Part>, }} */
export function opt_span_origin_erase(span) {
  return /** @type {{ span: Opt<Span<Origin<Erased, $Part>>>, eraser: Origin_eraser<$Origin, $Part> }} */ (
    span
  );
}
/** @template $Origin, $Part @param {{ span: Span<Origin<Erased, $Part>>, uneraser: Origin_uneraser<$Origin, $Part>, }} span @returns {{ span: Span<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin, $Part>, }} */
export function span_origin_unerase(span) {
  return /** @type {{ span: Span<Origin<$Origin, $Part>>, uneraser: Origin_uneraser<$Origin, $Part> }} */ (
    span
  );
}
/** @template $Origin, $Part @param {{ span: Opt<Span<Origin<Erased, $Part>>>, uneraser: Origin_uneraser<$Origin, $Part>, }} span @returns {{ span: Opt<Span<Origin<$Origin, $Part>>>, uneraser: Origin_uneraser<$Origin, $Part>, }} */
export function opt_span_origin_unerase(span) {
  return /** @type {{ span: Opt<Span<Origin<$Origin, $Part>>>, uneraser: Origin_uneraser<$Origin, $Part> }} */ (
    span
  );
}
/** @template $Origin @param {Span<$Origin>} span @returns {{ span: Span<$Origin>, length: P32, }} */
export function span_length(span) {
  return { span: span, length: span.length };
}
/** @template $Origin @param {Opt<Span<$Origin>>} span @returns {{ span: Opt<Span<$Origin>>, length: U32, }} */
export function opt_span_length(span) {
  return { span: span, length: "no" in span ? 0 : span.yes.length };
}
/** @template $Origin @param {Unset_span<$Origin>} span @returns {{ span: Unset_span<$Origin>, length: P32, }} */
export function unset_span_length(span) {
  return span_length(span);
}
/** @template $Origin @param {Opt<Unset_span<$Origin>>} span @returns {{ span: Opt<Unset_span<$Origin>>, length: U32, }} */
export function opt_unset_span_length(span) {
  return opt_span_length(span);
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
/** @template $Origin @param {Unset_span<$Origin>} span @returns {{ end: Slot<$Origin>, before: Opt<Unset_span<$Origin>>, }} */
export function unset_span_end(span) {
  return span_end(span);
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
/** @template $Origin @param {Unset_span<$Origin>} unset_span @returns {{ start: Slot<$Origin>, after: Opt<Unset_span<$Origin>>, }} */
export function unset_span_start(unset_span) {
  return span_start(unset_span);
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
/** @template $Origin @param {{ span: Unset_span<$Origin>, length: P32, }} take @returns {{ start: Unset_span<$Origin>, after: Opt<Unset_span<$Origin>>, }} */
export function unset_span_start_of_length_positive(take) {
  return span_start_of_length_positive(take);
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
/** @template $Origin @param {{ span: Unset_span<$Origin>, length: P32, }} unset_span @returns{{ end: Unset_span<$Origin>, before: Opt<Unset_span<$Origin>>, }} */
export function unset_span_end_of_length_positive(unset_span) {
  return span_end_of_length_positive(unset_span);
}
/** @template $Origin, $State @param {{ span: Span<$Origin>, direction: { up: void } | { down: void }, state: $State, step: Fn<{ slot: Slot<$Origin>, state: $State, }, $State>, }} fold @returns {$State} */
export function span_fold(fold) {
  let state = fold.state;
  if ("up" in fold.direction) {
    for (let i = fold.span.start; i < fold.span.start + fold.span.length; i++) {
      state = fold.step({ state: state, slot: i });
    }
  } else {
    for (let i = fold.span.start + fold.span.length - 1; i >= fold.span.start; i--) {
      state = fold.step({ state: state, slot: i });
    }
  }
  return state;
}
/** @template $Origin, $State @param {{ span: Opt<Span<$Origin>>, direction: { up: void } | { down: void }, state: $State, step: Fn<{ slot: Slot<$Origin>, state: $State, }, $State>, }} fold @returns {$State} */
export function opt_span_fold(fold) {
  if ("no" in fold.span) return fold.state;
  return span_fold({
    span: fold.span.yes,
    direction: fold.direction,
    state: fold.state,
    step: fold.step,
  });
}
/** @template $Origin, $State @param {{ span: Unset_span<$Origin>, direction: { up: void } | { down: void }, state: $State, step: Fn<{ slot: Unset_slot<$Origin>, state: $State, }, $State>, }} fold @returns {$State} */
export function unset_span_fold(fold) {
  return span_fold(fold);
}
/** @template $Origin, $State @param {{ span: Opt<Unset_span<$Origin>>, direction: { up: void } | { down: void }, state: $State, step: Fn<{ slot: Unset_slot<$Origin>, state: $State, }, $State>, }} fold @returns {$State} */
export function opt_unset_span_fold(fold) {
  return opt_span_fold(fold);
}

/** @template $Element, $Origin, $Part @param {Origin<$Origin, $Part>} _ @returns {Buf<$Origin, $Element>} */
export function buf_empty(_) {
  return [];
}
/** @template $Element, $Origin @param {Buf<$Origin, $Element>} _ @returns {void} */
export function buf_rid(_) {}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, slot: Unset_slot<$Origin>, }} rid @returns {Buf<$Origin, $Element>} */
export function buf_slot_rid(rid) {
  if (rid.slot + 1 === rid.buf.length) {
    rid.buf.pop();
  } else {
    rid.buf[rid.slot] = null;
  }
  return rid.buf;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Unset_span<$Origin>, }} rid @returns {Buf<$Origin, $Element>} */
export function buf_span_rid(rid) {
  if (rid.span.start + rid.span.length === rid.buf.length) {
    rid.buf.length -= rid.span.length;
  } else {
    for (let i = rid.span.start; i < rid.span.start + rid.span.length; i++) {
      rid.buf[i] = null;
    }
  }
  return rid.buf;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Opt<Unset_span<$Origin>>, }} rid @returns {Buf<$Origin, $Element>} */
export function buf_opt_span_rid(rid) {
  if ("yes" in rid.span) {
    for (let i = rid.span.yes.start; i < rid.span.yes.start + rid.span.yes.length; i++) {
      rid.buf[i] = null;
    }
  }
  return rid.buf;
}
/** @template $Element, $Origin @param {Buf<$Origin, $Element>} buf @returns {Buf<$Origin, $Element>} */
export function buf_pre_allocation_rid(buf) {
  return buf;
}
/** @template $Element, $Origin, $Part @param {{ buf: Buf<Origin<$Origin, $Part>, $Element>, eraser: Origin_eraser<$Origin, $Part>, }} erase @returns {Buf<Origin<Erased, $Part>, $Element>} */
export function buf_origin_erase(erase) {
  return /** @type Buf<Origin<Erased, $Part>, $Element> */ (erase.buf);
}
/** @template $Element, $Origin, $Part @param {{ buf: Buf<Origin<Erased, $Part>, $Element>, uneraser: Origin_uneraser<$Origin, $Part>, }} unerase @returns {Buf<Origin<$Origin, $Part>, $Element>} */
export function buf_origin_unerase(unerase) {
  return /** @type Buf<Origin<$Origin, $Part>, $Element> */ (unerase.buf);
}
/** @template $Element, $Element_erased, $Origin, $Part @param {{ buf: Buf<Origin<$Origin, $Part>, $Element>, eraser: Origin_eraser<$Origin, $Part>, element_erase: Fn<{ element: $Element, eraser: Origin_eraser<$Origin, $Part>, }, { element: $Element_erased, eraser: Origin_eraser<$Origin, $Part>, }>, }} erase @returns {Buf<Origin<Erased, $Part>, $Element_erased>} */
export function buf_origin_erase_with_elements(erase) {
  return erase.buf.map((element) =>
    element === null
      ? null
      : erase.element_erase({
          element: element,
          eraser: erase.eraser,
        }).element,
  );
}
/** @template $Element, $Element_erased, $Origin, $Part @param {{ buf: Buf<Origin<Erased, $Part>, $Element_erased>, uneraser: Origin_uneraser<$Origin, $Part>, element_unerase: Fn<{ element: $Element_erased, uneraser: Origin_uneraser<$Origin, $Part>, }, { element: $Element, uneraser: Origin_uneraser<$Origin, $Part>, }>, }} unerase @returns {Buf<Origin<$Origin, $Part>, $Element>} */
export function buf_origin_unerase_with_elements(unerase) {
  return unerase.buf.map((element) =>
    element === null
      ? null
      : unerase.element_unerase({
          element: element,
          uneraser: unerase.uneraser,
        }).element,
  );
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, element: $Element, slot: Slot<$Origin>, }} unset @returns {{ buf: Buf<$Origin, $Element>, slot: Unset_slot<$Origin>, element: $Element, }} */
export function buf_unset(unset) {
  return {
    buf: unset.buf,
    slot: unset.slot,
    element: /** @type $Element */ (unset.buf[unset.slot]),
  };
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, slot: Unset_slot<$Origin>, newø: $Element, }} set @returns {{ buf: Buf<$Origin, $Element>, slot: Unset_slot<$Origin>, }} */
export function buf_set(set) {
  set.buf[set.slot] = set.newø;
  return { buf: set.buf, slot: set.slot };
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, length: U32, }} pre_allocate @returns {Buf<$Origin, $Element>} */
export function buf_pre_allocate_at_least(pre_allocate) {
  // There seems to be no way which does not also influence the length.
  // Maybe .length +=; followed by .length -=; does the job?
  return pre_allocate.buf;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, newø: $Element, }} add @returns {{ buf: Buf<$Origin, $Element>, slot: Slot<$Origin>, }} */
export function buf_add(add) {
  let new_index = add.buf.length;
  add.buf.push(add.newø);
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return { buf: add.buf, slot: new_index };
}
/** @template $Element, $Origin @param {Buf<$Origin, $Element>} buf @returns {{ buf: Buf<$Origin, $Element>, slot: Unset_slot<$Origin>, }} */
export function buf_add_unset(buf) {
  let new_index = buf.length;
  buf.push(null);
  if (buf.length > U32$MAX)
    throw Error("Array length " + buf.length + " not representable as a u32");
  return { buf: buf, slot: new_index };
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, newø: $Element, }} insert @returns {{ buf: Buf<$Origin, $Element>, slot: Slot<$Origin>, }} */
export function buf_insert(insert) {
  const existing_vacant_index = insert.buf.findIndex((el) => el === null);
  if (existing_vacant_index >= 0) {
    insert.buf[existing_vacant_index] = insert.newø;
    return { buf: insert.buf, slot: existing_vacant_index };
  } else {
    let new_index = insert.buf.length;
    insert.buf.push(insert.newø);
    if (insert.buf.length > U32$MAX)
      throw Error("Array length " + insert.buf.length + " not representable as a u32");
    return { buf: insert.buf, slot: new_index };
  }
}
/** @template $Element, $Origin @param {Buf<$Origin, $Element>} buf @returns {{ buf: Buf<$Origin, $Element>, slot: Unset_slot<$Origin>, }} */
export function buf_insert_unset(buf) {
  const existing_vacant_index = buf.findIndex((el) => el === null);
  if (existing_vacant_index >= 0) {
    buf[existing_vacant_index] = null;
    return { buf: buf, slot: existing_vacant_index };
  } else {
    let new_index = buf.length;
    buf.push(null);
    if (buf.length > U32$MAX)
      throw Error("Array length " + buf.length + " not representable as a u32");
    return { buf: buf, slot: new_index };
  }
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, length: U32, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Opt<Unset_span<$Origin>>, }} */
export function buf_add_unset_length(add) {
  if (add.length === 0) return { buf: add.buf, span: { no: undefined } };
  let start = add.buf.length;
  for (let count = 0; count < add.length; count++) {
    add.buf.push(null);
  }
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: { yes: { start: start, length: add.length } },
  };
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, length: P32, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Unset_span<$Origin>, }} */
export function buf_add_unset_length_positive(add) {
  let start = add.buf.length;
  for (let count = 0; count < add.length; count++) {
    add.buf.push(null);
  }
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return { buf: add.buf, span: { start: start, length: add.length } };
}
/** @template $Element, $Origin @template $Record @param {{ buf: Buf<$Origin, $Element>, newø: Array<$Element, $Record>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
export function buf_add_array(add) {
  add.buf.push(...add.newø);
  if (add.buf.length > U32$MAX)
    throw Error("Array length " + add.buf.length + " not representable as a u32");
  return {
    buf: add.buf,
    span: { start: add.buf.length - add.newø.length, length: add.newø.length },
  };
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, newø: $Element, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
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
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, newø: $Element, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
export function buf_opt_span_add(add) {
  if ("no" in add.span) {
    let new_index = add.buf.length;
    add.buf.push(add.newø);
    if (add.buf.length > U32$MAX)
      throw Error("Array length " + add.buf.length + " not representable as a u32");
    return { buf: add.buf, span: slot_to_span(new_index) };
  }
  return buf_span_add({ buf: add.buf, span: add.span.yes, newø: add.newø });
}
/** @template $Element, $Origin @template $Record @param {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, newø: Array<$Element, $Record>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
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
/** @template $Element, $Origin @template $Record @param {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, newø: Array<$Element, $Record>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
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
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, slot: Slot<$Origin>, }} remove @returns {{ buf: Buf<$Origin, $Element>, element: $Element, }} */
export function buf_remove(remove) {
  let element = /** @type {$Element} */ (remove.buf[remove.slot]);
  remove.buf[remove.slot] = null;
  return { buf: remove.buf, element: element };
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} move @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
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
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} move @returns {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} */
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
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} move @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
export function buf_span_move_to_vacant(move) {
  if (move.span.start + move.span.length < move.buf.length) return move;
  let vacant_length = 0;
  for (let i = 0; i < move.buf.length; i++) {
    if (move.buf[i] === null) {
      vacant_length++;
      if (vacant_length === move.span.length) {
        const vacant_start = i - move.span.length + 1;
        for (let vacant_i = 0; vacant_i < move.span.length; vacant_i++) {
          move.buf[vacant_start + vacant_i] = move.buf[move.span.start + vacant_i];
        }
        move.buf.length -= move.span.length;
        return { buf: move.buf, span: { start: vacant_start, length: move.span.length } };
      }
    } else {
      vacant_length = 0;
    }
  }
  return move;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} move @returns {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} */
export function buf_opt_span_move_to_vacant(move) {
  if ("no" in move.span) return move;
  const span = move.span.yes;
  if (span.start + span.length < move.buf.length) return move;
  let vacant_length = 0;
  for (let i = 0; i < move.buf.length; i++) {
    if (move.buf[i] === null) {
      vacant_length++;
      if (vacant_length === span.length) {
        const vacant_start = i - span.length + 1;
        for (let vacant_i = 0; vacant_i < span.length; vacant_i++) {
          move.buf[vacant_start + vacant_i] = move.buf[span.start + vacant_i];
        }
        move.buf.length -= span.length;
        return {
          buf: move.buf,
          span: { yes: { start: vacant_start, length: span.length } },
        };
      }
    } else {
      vacant_length = 0;
    }
  }
  return move;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} reverse @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
export function buf_span_reverse(reverse) {
  const slice = reverse.buf.slice(
    reverse.span.start,
    reverse.span.start + reverse.span.length,
  );
  slice.reverse();
  reverse.buf.splice(reverse.span.start, reverse.span.length, ...slice);
  return reverse;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} reverse @returns {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} */
export function buf_opt_span_reverse(reverse) {
  if ("no" in reverse.span) return reverse;
  const span = reverse.span.yes;
  const slice = reverse.buf.slice(span.start, span.start + span.length);
  slice.reverse();
  reverse.buf.splice(span.start, span.length, ...slice);
  return reverse;
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, start: Span<$Origin>, end: Span<$Origin>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }}} */
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
  // move end span to end after the start elements
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
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, start: Span<$Origin>, end: Opt<Span<$Origin>>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
export function buf_span_add_own_opt_span(add) {
  if ("no" in add.end) {
    return { buf: add.buf, span: add.start };
  }
  return buf_span_add_own_span({ buf: add.buf, start: add.start, end: add.end.yes });
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, start: Opt<Span<$Origin>>, end: Span<$Origin>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Span<$Origin>, }} */
export function buf_opt_span_add_own_span(add) {
  if ("no" in add.start) {
    return { buf: add.buf, span: add.end };
  }
  return buf_span_add_own_span({ buf: add.buf, start: add.start.yes, end: add.end });
}
/** @template $Element, $Origin @param {{ buf: Buf<$Origin, $Element>, start: Opt<Span<$Origin>>, end: Opt<Span<$Origin>>, }} add @returns {{ buf: Buf<$Origin, $Element>, span: Opt<Span<$Origin>>, }} */
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
/** @template $Element, $Origin @param {Buf<$Origin, $Element>} buf @returns {Unset_slice<$Element>} */
export function buf_to_unset(buf) {
  return buf;
}
/** @template $Element, $Origin, $Part @param {{ origin: Origin<$Origin, $Part>, slice: Unset_slice<$Element>, }} reuse @returns {Buf<$Origin, $Element>} */
export function buf_reuse(reuse) {
  reuse.slice.length = 0;
  return reuse.slice;
}

/** @template $Element @param {Unset_slice<$Element>} _ @returns {void} */
export function unset_slice_rid(_) {}
/** @template $Element @param {Unset_slice<$Element>} unset_slice @returns {{ slice: Unset_slice<$Element>, length: U32, }} */
export function unset_slice_length(unset_slice) {
  return { slice: unset_slice, length: unset_slice.length };
}
/** @template $Element @param {U32} length @returns {Unset_slice<$Element>} */
export function unset_slice_allocate_length(length) {
  return Array(length).fill(null);
}
/** @template $Element, $New_element @param {Unset_slice<$Element>} unset_slice @returns {Unset_slice<$New_element>} */
export function unset_slice_cast_or_rid_and_allocate(unset_slice) {
  // for once equal type sizes comes in clutch
  return /** @type ($New_element | null)[] */ (unset_slice);
}
