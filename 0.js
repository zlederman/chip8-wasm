(window.webpackJsonp=window.webpackJsonp||[]).push([[0],[,,function(t,n,r){"use strict";(function(t,_){let e;function i(t){e=t}r.d(n,"A",(function(){return i})),r.d(n,"d",(function(){return O})),r.d(n,"c",(function(){return j})),r.d(n,"a",(function(){return q})),r.d(n,"b",(function(){return F})),r.d(n,"n",(function(){return A})),r.d(n,"q",(function(){return E})),r.d(n,"B",(function(){return T})),r.d(n,"i",(function(){return z})),r.d(n,"L",(function(){return N})),r.d(n,"j",(function(){return P})),r.d(n,"w",(function(){return D})),r.d(n,"h",(function(){return I})),r.d(n,"G",(function(){return U})),r.d(n,"v",(function(){return C})),r.d(n,"D",(function(){return J})),r.d(n,"u",(function(){return L})),r.d(n,"H",(function(){return B})),r.d(n,"o",(function(){return M})),r.d(n,"x",(function(){return G})),r.d(n,"F",(function(){return H})),r.d(n,"M",(function(){return K})),r.d(n,"r",(function(){return R})),r.d(n,"g",(function(){return S})),r.d(n,"K",(function(){return V})),r.d(n,"y",(function(){return Q})),r.d(n,"E",(function(){return W})),r.d(n,"k",(function(){return X})),r.d(n,"l",(function(){return Y})),r.d(n,"I",(function(){return Z})),r.d(n,"f",(function(){return $})),r.d(n,"e",(function(){return tt})),r.d(n,"s",(function(){return nt})),r.d(n,"p",(function(){return rt})),r.d(n,"z",(function(){return _t})),r.d(n,"m",(function(){return et})),r.d(n,"t",(function(){return it})),r.d(n,"C",(function(){return ot})),r.d(n,"N",(function(){return ut})),r.d(n,"J",(function(){return ct}));const o=new Array(128).fill(void 0);function u(t){return o[t]}o.push(void 0,null,!0,!1);let c=o.length;function s(t){const n=u(t);return function(t){t<132||(o[t]=c,c=t)}(t),n}let f=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let p=null;function a(){return null!==p&&0!==p.byteLength||(p=new Uint8Array(e.memory.buffer)),p}function w(t,n){return t>>>=0,f.decode(a().subarray(t,t+n))}function d(t){c===o.length&&o.push(o.length+1);const n=c;return c=o[n],o[n]=t,n}let g=128;let h=null;function b(){return null!==h&&0!==h.byteLength||(h=new Int32Array(e.memory.buffer)),h}let l=0;let y=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const m="function"==typeof y.encodeInto?function(t,n){return y.encodeInto(t,n)}:function(t,n){const r=y.encode(t);return n.set(r),{read:t.length,written:r.length}};function v(t,n,r){if(void 0===r){const r=y.encode(t),_=n(r.length,1)>>>0;return a().subarray(_,_+r.length).set(r),l=r.length,_}let _=t.length,e=n(_,1)>>>0;const i=a();let o=0;for(;o<_;o++){const n=t.charCodeAt(o);if(n>127)break;i[e+o]=n}if(o!==_){0!==o&&(t=t.slice(o)),e=r(e,_,_=o+3*t.length,1)>>>0;const n=a().subarray(e+o,e+_);o+=m(t,n).written}return l=o,e}function k(t,n){if(!(t instanceof n))throw new Error("expected instance of "+n.name);return t.ptr}function x(t,n){try{return t.apply(this,n)}catch(t){e.__wbindgen_exn_store(d(t))}}const O=Object.freeze({ON:1,1:"ON",OFF:0,0:"OFF"}),j=Object.freeze({ON:1,1:"ON",OFF:0,0:"OFF"});class q{static __wrap(t){t>>>=0;const n=Object.create(q.prototype);return n.__wbg_ptr=t,n}__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,t}free(){const t=this.__destroy_into_raw();e.__wbg_chip8_free(t)}get delay_timer(){return e.__wbg_get_chip8_delay_timer(this.__wbg_ptr)}set delay_timer(t){e.__wbg_set_chip8_delay_timer(this.__wbg_ptr,t)}get sound_timer(){return e.__wbg_get_chip8_sound_timer(this.__wbg_ptr)}set sound_timer(t){e.__wbg_set_chip8_sound_timer(this.__wbg_ptr,t)}static new(t){try{const n=e.chip8_new(function(t){if(1==g)throw new Error("out of js stack");return o[--g]=t,g}(t));return q.__wrap(n)}finally{o[g++]=void 0}}get_display(){return e.chip8_get_display(this.__wbg_ptr)}tick(){e.chip8_tick(this.__wbg_ptr)}tick_timers(){e.chip8_tick_timers(this.__wbg_ptr)}get_pc(){return e.chip8_get_pc(this.__wbg_ptr)>>>0}get_top_of_stack(){return e.chip8_get_top_of_stack(this.__wbg_ptr)>>>0}get_register(t){return e.chip8_get_register(this.__wbg_ptr,t)}get_index(){return e.chip8_get_index(this.__wbg_ptr)>>>0}get_mem_at(t){return e.chip8_get_mem_at(this.__wbg_ptr,t)}set_key_state(t,n){e.chip8_set_key_state(this.__wbg_ptr,t,n)}get_key_state(t){return e.chip8_get_key_state(this.__wbg_ptr,t)>>>0}eight(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_eight(this.__wbg_ptr,n)}zero(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_zero(this.__wbg_ptr,n)}f(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_f(this.__wbg_ptr,n)}push(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_push(this.__wbg_ptr,n)}pop(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_pop(this.__wbg_ptr,n)}skip_if_eq(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_skip_if_eq(this.__wbg_ptr,n)}skip_if_neq(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_skip_if_neq(this.__wbg_ptr,n)}skip_eq_reg(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_skip_eq_reg(this.__wbg_ptr,n)}skip_neq_reg(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_skip_neq_reg(this.__wbg_ptr,n)}skip_key(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_skip_key(this.__wbg_ptr,n)}clear(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_clear(this.__wbg_ptr,n)}jump(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_jump(this.__wbg_ptr,n)}offset_jump(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_offset_jump(this.__wbg_ptr,n)}set_register(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_set_register(this.__wbg_ptr,n)}add_register(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_add_register(this.__wbg_ptr,n)}set_index(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_set_index(this.__wbg_ptr,n)}add_to_index(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_add_to_index(this.__wbg_ptr,n)}timers(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_timers(this.__wbg_ptr,n)}get_key(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_get_key(this.__wbg_ptr,n)}get_font(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_get_font(this.__wbg_ptr,n)}store(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_store(this.__wbg_ptr,n)}load(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_load(this.__wbg_ptr,n)}decimal_conversion(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_decimal_conversion(this.__wbg_ptr,n)}stateful_arithmetic(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_stateful_arithmetic(this.__wbg_ptr,n)}random(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_random(this.__wbg_ptr,n)}draw(t){k(t,F);var n=t.__destroy_into_raw();e.chip8_draw(this.__wbg_ptr,n)}}class F{static __wrap(t){t>>>=0;const n=Object.create(F.prototype);return n.__wbg_ptr=t,n}__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,t}free(){const t=this.__destroy_into_raw();e.__wbg_instruction_free(t)}get operation(){return e.__wbg_get_instruction_operation(this.__wbg_ptr)}set operation(t){e.__wbg_set_instruction_operation(this.__wbg_ptr,t)}get x(){return e.__wbg_get_instruction_x(this.__wbg_ptr)}set x(t){e.__wbg_set_instruction_x(this.__wbg_ptr,t)}get y(){return e.__wbg_get_instruction_y(this.__wbg_ptr)}set y(t){e.__wbg_set_instruction_y(this.__wbg_ptr,t)}get n(){return e.__wbg_get_instruction_n(this.__wbg_ptr)}set n(t){e.__wbg_set_instruction_n(this.__wbg_ptr,t)}get nn(){return e.__wbg_get_instruction_nn(this.__wbg_ptr)}set nn(t){e.__wbg_set_instruction_nn(this.__wbg_ptr,t)}get nnn(){return e.__wbg_get_instruction_nnn(this.__wbg_ptr)}set nnn(t){e.__wbg_set_instruction_nnn(this.__wbg_ptr,t)}to_string(){let t,n;try{const i=e.__wbindgen_add_to_stack_pointer(-16);e.instruction_to_string(i,this.__wbg_ptr);var r=b()[i/4+0],_=b()[i/4+1];return t=r,n=_,w(r,_)}finally{e.__wbindgen_add_to_stack_pointer(16),e.__wbindgen_free(t,n,1)}}static from_str(t){const n=v(t,e.__wbindgen_malloc,e.__wbindgen_realloc),r=l,_=e.instruction_from_str(n,r);return F.__wrap(_)}}function A(t,n){console.log(w(t,n))}function E(){return d(new Error)}function T(t,n){const r=v(u(n).stack,e.__wbindgen_malloc,e.__wbindgen_realloc),_=l;b()[t/4+1]=_,b()[t/4+0]=r}function z(t,n){let r,_;try{r=t,_=n,console.error(w(t,n))}finally{e.__wbindgen_free(r,_,1)}}function N(t){s(t)}function P(){return x((function(t,n){u(t).getRandomValues(u(n))}),arguments)}function D(){return x((function(t,n){u(t).randomFillSync(s(n))}),arguments)}function I(t){return d(u(t).crypto)}function U(t){const n=u(t);return"object"==typeof n&&null!==n}function C(t){return d(u(t).process)}function J(t){return d(u(t).versions)}function L(t){return d(u(t).node)}function B(t){return"string"==typeof u(t)}function M(t){return d(u(t).msCrypto)}function G(){return x((function(){return d(t.require)}),arguments)}function H(t){return"function"==typeof u(t)}function K(t,n){return d(w(t,n))}function R(t,n){return d(new Function(w(t,n)))}function S(){return x((function(t,n){return d(u(t).call(u(n)))}),arguments)}function V(t){return d(u(t))}function Q(){return x((function(){return d(self.self)}),arguments)}function W(){return x((function(){return d(window.window)}),arguments)}function X(){return x((function(){return d(globalThis.globalThis)}),arguments)}function Y(){return x((function(){return d(_.global)}),arguments)}function Z(t){return void 0===u(t)}function $(){return x((function(t,n,r){return d(u(t).call(u(n),u(r)))}),arguments)}function tt(t){return d(u(t).buffer)}function nt(t,n,r){return d(new Uint8Array(u(t),n>>>0,r>>>0))}function rt(t){return d(new Uint8Array(u(t)))}function _t(t,n,r){u(t).set(u(n),r>>>0)}function et(t){return u(t).length}function it(t){return d(new Uint8Array(t>>>0))}function ot(t,n,r){return d(u(t).subarray(n>>>0,r>>>0))}function ut(t,n){throw new Error(w(t,n))}function ct(){return d(e.memory)}}).call(this,r(3)(t),r(4))},function(t,n){t.exports=function(t){if(!t.webpackPolyfill){var n=Object.create(t);n.children||(n.children=[]),Object.defineProperty(n,"loaded",{enumerable:!0,get:function(){return n.l}}),Object.defineProperty(n,"id",{enumerable:!0,get:function(){return n.i}}),Object.defineProperty(n,"exports",{enumerable:!0}),n.webpackPolyfill=1}return n}},function(t,n){var r;r=function(){return this}();try{r=r||new Function("return this")()}catch(t){"object"==typeof window&&(r=window)}t.exports=r},function(t,n,r){"use strict";var _=r.w[t.i];for(var e in r.r(n),_)"__webpack_init__"!=e&&(n[e]=_[e]);r(2);_.__webpack_init__()}]]);