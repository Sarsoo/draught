(self.webpackChunkdraught=self.webpackChunkdraught||[]).push([[10],{171:(t,r,e)=>{"use strict";e.a(t,(async n=>{e.d(r,{QC:()=>j,SZ:()=>O,lA:()=>q,Ck:()=>C,ug:()=>I,h4:()=>A,h9:()=>W,Dz:()=>U,kF:()=>M,s8:()=>$,WB:()=>R,cP:()=>D,a:()=>P,l_:()=>z,U_:()=>F,EN:()=>J,qu:()=>Q,Wg:()=>N,B3:()=>K,I6:()=>L,Qb:()=>X,cI:()=>Y,QK:()=>H,Md:()=>Z,ox:()=>G,ES:()=>V,Ae:()=>tt,UL:()=>rt,qw:()=>et,tS:()=>nt,R$:()=>_t,md:()=>at,IF:()=>it,XP:()=>ot,m_:()=>ct,fY:()=>st,Or:()=>ut});var _=e(813);t=e.hmd(t);var a=n([_]);_=(a.then?await a:a)[0];const i=new Array(32).fill(void 0);function o(t){return i[t]}i.push(void 0,null,!0,!1);let c=i.length;let s=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let u=null;function p(){return null!==u&&u.buffer===_.memory.buffer||(u=new Uint8Array(_.memory.buffer)),u}function l(t,r){return s.decode(p().subarray(t,t+r))}function d(t){c===i.length&&i.push(i.length+1);const r=c;return c=i[r],i[r]=t,r}function b(t){const r=typeof t;if("number"==r||"boolean"==r||null==t)return`${t}`;if("string"==r)return`"${t}"`;if("symbol"==r){const r=t.description;return null==r?"Symbol":`Symbol(${r})`}if("function"==r){const r=t.name;return"string"==typeof r&&r.length>0?`Function(${r})`:"Function"}if(Array.isArray(t)){const r=t.length;let e="[";r>0&&(e+=b(t[0]));for(let n=1;n<r;n++)e+=", "+b(t[n]);return e+="]",e}const e=/\[object ([^\]]+)\]/.exec(toString.call(t));let n;if(!(e.length>1))return toString.call(t);if(n=e[1],"Object"==n)try{return"Object("+JSON.stringify(t)+")"}catch(t){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:n}let g=0,f=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const w="function"==typeof f.encodeInto?function(t,r){return f.encodeInto(t,r)}:function(t,r){const e=f.encode(t);return r.set(e),{read:t.length,written:e.length}};function h(t,r,e){if(void 0===e){const e=f.encode(t),n=r(e.length);return p().subarray(n,n+e.length).set(e),g=e.length,n}let n=t.length,_=r(n);const a=p();let i=0;for(;i<n;i++){const r=t.charCodeAt(i);if(r>127)break;a[_+i]=r}if(i!==n){0!==i&&(t=t.slice(i)),_=e(_,n,n=i+3*t.length);const r=p().subarray(_+i,_+n);i+=w(t,r).written}return g=i,_}let m=null;function v(){return null!==m&&m.buffer===_.memory.buffer||(m=new Int32Array(_.memory.buffer)),m}function y(t){return null==t}function x(t,r){if(!(t instanceof r))throw new Error(`expected instance of ${r.name}`);return t.ptr}function j(){_.init_game()}function k(t,r){try{return t.apply(this,r)}catch(t){_.__wbindgen_exn_store(d(t))}}Object.freeze({Move:0,0:"Move",Jump:1,1:"Jump"});const O=Object.freeze({Black:0,0:"Black",White:1,1:"White"});Object.freeze({Man:0,0:"Man",King:1,1:"King"}),Object.freeze({Empty:0,0:"Empty",Occupied:1,1:"Occupied",Unplayable:2,2:"Unplayable"}),Object.freeze({Allowed:0,0:"Allowed",UnoccupiedSrc:1,1:"UnoccupiedSrc",OccupiedDest:2,2:"OccupiedDest",OutOfBounds:3,3:"OutOfBounds",Unplayable:4,4:"Unplayable",WrongTeamSrc:5,5:"WrongTeamSrc",IllegalTrajectory:6,6:"IllegalTrajectory",NoJumpablePiece:7,7:"NoJumpablePiece",JumpingSameTeam:8,8:"JumpingSameTeam"});class S{static __wrap(t){const r=Object.create(S.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_board_free(t)}get width(){return _.__wbg_get_board_width(this.ptr)>>>0}set width(t){_.__wbg_set_board_width(this.ptr,t)}get height(){return _.__wbg_get_board_height(this.ptr)>>>0}set height(t){_.__wbg_set_board_height(this.ptr,t)}get current_turn(){return _.__wbg_get_board_current_turn(this.ptr)>>>0}set current_turn(t){_.__wbg_set_board_current_turn(this.ptr,t)}cell(t){var r=_.board_cell(this.ptr,t);return B.__wrap(r)}set_cell(t,r){x(r,B);var e=r.ptr;r.ptr=0,_.board_set_cell(this.ptr,t,e)}grid_cell(t){x(t,E);var r=t.ptr;t.ptr=0;var e=_.board_grid_cell(this.ptr,r);return B.__wrap(e)}cell_index(t,r){return _.board_cell_index(this.ptr,t,r)>>>0}cell_idx(t){x(t,E);var r=t.ptr;return t.ptr=0,_.board_cell_idx(this.ptr,r)>>>0}board_index(t){var r=_.board_board_index(this.ptr,t);return E.__wrap(r)}can_move(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;return r.ptr=0,_.board_can_move(this.ptr,e,n)>>>0}validate_man_move(t,r,e){x(t,E);var n=t.ptr;t.ptr=0,x(r,E);var a=r.ptr;r.ptr=0,x(e,T);var i=e.ptr;return e.ptr=0,_.board_validate_man_move(this.ptr,n,a,i)>>>0}validate_king_move(t,r,e){x(t,E);var n=t.ptr;t.ptr=0,x(r,E);var a=r.ptr;r.ptr=0,x(e,T);var i=e.ptr;return e.ptr=0,_.board_validate_king_move(this.ptr,n,a,i)>>>0}jumpee_idx(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;return r.ptr=0,_.board_jumpee_idx(this.ptr,e,n)>>>0}cells(){return _.board_cells(this.ptr)}num_cells(){return _.board_num_cells(this.ptr)>>>0}num_pieces(){return _.board_num_pieces(this.ptr)>>>0}num_player(t){return _.board_num_player(this.ptr,t)>>>0}score(){return _.board_score(this.ptr)}cell_state(t){return _.board_cell_state(this.ptr,t)>>>0}apply_move(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;r.ptr=0;var a=_.board_apply_move(this.ptr,e,n);return S.__wrap(a)}apply_jump(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;r.ptr=0;var a=_.board_apply_jump(this.ptr,e,n);return S.__wrap(a)}static validate_jumpee(t,r){x(t,B);var e=t.ptr;t.ptr=0,x(r,T);var n=r.ptr;return r.ptr=0,_.board_validate_jumpee(e,n)>>>0}static check_jumpee_team(t,r){x(t,T);var e=t.ptr;t.ptr=0,x(r,T);var n=r.ptr;return r.ptr=0,0!==_.board_check_jumpee_team(e,n)}constructor(t,r,e){var n=_.board_new(t,r,e);return S.__wrap(n)}static init_game(t,r){x(t,S);var e=t.ptr;t.ptr=0;var n=_.board_init_game(e,r);return S.__wrap(n)}}class E{static __wrap(t){const r=Object.create(E.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_brdidx_free(t)}get row(){return _.__wbg_get_brdidx_row(this.ptr)>>>0}set row(t){_.__wbg_set_brdidx_row(this.ptr,t)}get col(){return _.__wbg_get_brdidx_col(this.ptr)>>>0}set col(t){_.__wbg_set_brdidx_col(this.ptr,t)}static from(t,r){var e=_.brdidx_from(t,r);return E.__wrap(e)}}class q{static __wrap(t){const r=Object.create(q.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_game_free(t)}current_board_cells(){return _.game_current_board_cells(this.ptr)}current_board_len(){return _.game_current_board_len(this.ptr)>>>0}current_turn(){return _.game_current_turn(this.ptr)>>>0}make_move(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;r.ptr=0,_.game_make_move(this.ptr,e,n)}execute_move(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;r.ptr=0,_.game_execute_move(this.ptr,e,n)}execute_jump(t,r){x(t,E);var e=t.ptr;t.ptr=0,x(r,E);var n=r.ptr;r.ptr=0,_.game_execute_jump(this.ptr,e,n)}push_new_board(t){x(t,S);var r=t.ptr;t.ptr=0,_.game_push_new_board(this.ptr,r)}set_current(t){x(t,S);var r=t.ptr;t.ptr=0,_.game_set_current(this.ptr,r)}constructor(t,r,e,n){var a=_.game_new(t,r,e,n);return q.__wrap(a)}static new_with_canvas(t,r,e,n,a,i,o){var c=h(a,_.__wbindgen_malloc,_.__wbindgen_realloc),s=g,u=_.game_new_with_canvas(t,r,e,n,c,s,i,o);return q.__wrap(u)}set_painter(t){x(t,C);var r=t.ptr;t.ptr=0,_.game_set_painter(this.ptr,r)}draw(){_.game_draw(this.ptr)}}class C{static __wrap(t){const r=Object.create(C.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_painter_free(t)}constructor(t,r,e){var n=h(e,_.__wbindgen_malloc,_.__wbindgen_realloc),a=g,i=_.painter_new(t,r,n,a);return C.__wrap(i)}static new_with_canvas(t,r,e){var n=_.painter_new_with_canvas(t,r,d(e));return C.__wrap(n)}set_square_outline(t){_.painter_set_square_outline(this.ptr,d(t))}set_outline_width(t){_.painter_set_outline_width(this.ptr,t)}set_draw_outline(t){_.painter_set_draw_outline(this.ptr,t)}reset_dimensions(){_.painter_reset_dimensions(this.ptr)}validate_board_dim(t){return x(t,S),0!==_.painter_validate_board_dim(this.ptr,t.ptr)}draw(t){x(t,S),_.painter_draw(this.ptr,t.ptr)}}class T{static __wrap(t){const r=Object.create(T.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_piece_free(t)}get team(){return _.__wbg_get_piece_team(this.ptr)>>>0}set team(t){_.__wbg_set_piece_team(this.ptr,t)}get strength(){return _.__wbg_get_piece_strength(this.ptr)>>>0}set strength(t){_.__wbg_set_piece_strength(this.ptr,t)}constructor(t,r){var e=_.piece_new(t,r);return T.__wrap(e)}}class B{static __wrap(t){const r=Object.create(B.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_square_free(t)}get occupant(){var t=_.__wbg_get_square_occupant(this.ptr);return 0===t?void 0:T.__wrap(t)}set occupant(t){let r=0;y(t)||(x(t,T),r=t.ptr,t.ptr=0),_.__wbg_set_square_occupant(this.ptr,r)}get state(){return _.__wbg_get_square_state(this.ptr)>>>0}set state(t){_.__wbg_set_square_state(this.ptr,t)}constructor(t,r){let e=0;y(r)||(x(r,T),e=r.ptr,r.ptr=0);var n=_.square_new(t,e);return B.__wrap(n)}static pc(t,r){var e=_.square_pc(t,r);return B.__wrap(e)}static empty(){var t=_.square_empty();return B.__wrap(t)}static unplay(){var t=_.square_unplay();return B.__wrap(t)}}function I(t){!function(t){const r=o(t);(function(t){t<36||(i[t]=c,c=t)})(t)}(t)}function A(t,r){return d(l(t,r))}function W(){return d(new Error)}function U(t,r){var e=h(o(r).stack,_.__wbindgen_malloc,_.__wbindgen_realloc),n=g;v()[t/4+1]=n,v()[t/4+0]=e}function M(t,r){try{console.error(l(t,r))}finally{_.__wbindgen_free(t,r)}}function $(t){return o(t)instanceof Window}function R(t){var r=o(t).document;return y(r)?0:d(r)}function D(t,r,e){var n=o(t).getElementById(l(r,e));return y(n)?0:d(n)}function P(t){console.log(o(t))}function z(t){return o(t)instanceof CanvasRenderingContext2D}function F(t,r){o(t).strokeStyle=o(r)}function J(t,r){o(t).fillStyle=o(r)}function Q(t,r){o(t).lineWidth=r}function N(t){o(t).beginPath()}function K(t){o(t).fill()}function L(t){o(t).stroke()}function X(){return k((function(t,r,e,n,_,a){o(t).arc(r,e,n,_,a)}),arguments)}function Y(t,r,e,n,_){o(t).fillRect(r,e,n,_)}function H(t,r,e,n,_){o(t).strokeRect(r,e,n,_)}function Z(t){return o(t)instanceof HTMLCanvasElement}function G(t,r){o(t).width=r>>>0}function V(t,r){o(t).height=r>>>0}function tt(){return k((function(t,r,e){var n=o(t).getContext(l(r,e));return y(n)?0:d(n)}),arguments)}function rt(t,r){return d(new Function(l(t,r)))}function et(){return k((function(t,r){return d(o(t).call(o(r)))}),arguments)}function nt(){return k((function(){return d(self.self)}),arguments)}function _t(){return k((function(){return d(window.window)}),arguments)}function at(){return k((function(){return d(globalThis.globalThis)}),arguments)}function it(){return k((function(){return d(e.g.global)}),arguments)}function ot(t){return void 0===o(t)}function ct(t){return d(o(t))}function st(t,r){var e=h(b(o(r)),_.__wbindgen_malloc,_.__wbindgen_realloc),n=g;v()[t/4+1]=n,v()[t/4+0]=e}function ut(t,r){throw new Error(l(t,r))}}))},813:(t,r,e)=>{"use strict";var n=([n])=>e.v(r,t.id,"dfc37aa6c5fb55420dbe",{"./draught_bg.js":{__wbindgen_object_drop_ref:n.ug,__wbindgen_string_new:n.h4,__wbg_new_59cb74e423758ede:n.h9,__wbg_stack_558ba5917b466edd:n.Dz,__wbg_error_4bb6c2a97407129a:n.kF,__wbg_instanceof_Window_11e25482011fc506:n.s8,__wbg_document_5aff8cd83ef968f5:n.WB,__wbg_getElementById_b180ea4ada06a837:n.cP,__wbg_log_9a99fb1af846153b:n.a,__wbg_instanceof_CanvasRenderingContext2d_779e79c4121aa91b:n.l_,__wbg_setstrokeStyle_2939ee453716e462:n.U_,__wbg_setfillStyle_af790b5baf4d3210:n.EN,__wbg_setlineWidth_3e6b1837ae38d099:n.qu,__wbg_beginPath_2378575e37027ad3:n.Wg,__wbg_fill_558339447ed6ad43:n.B3,__wbg_stroke_c1e0313c58997dcf:n.I6,__wbg_arc_fffd87d9113dce32:n.Qb,__wbg_fillRect_46ffc8d2cef7e298:n.cI,__wbg_strokeRect_7ea34fad8a7f0fe2:n.QK,__wbg_instanceof_HtmlCanvasElement_fd3cbbe3906d7792:n.Md,__wbg_setwidth_f3c88eb520ba8d47:n.ox,__wbg_setheight_5a1abba41e35c42a:n.ES,__wbg_getContext_813df131fcbd6e91:n.Ae,__wbg_newnoargs_9fdd8f3961dd1bee:n.UL,__wbg_call_ba36642bd901572b:n.qw,__wbg_self_bb69a836a72ec6e9:n.tS,__wbg_window_3304fc4b414c9693:n.R$,__wbg_globalThis_e0d21cabc6630763:n.md,__wbg_global_8463719227271676:n.IF,__wbindgen_is_undefined:n.XP,__wbindgen_object_clone_ref:n.m_,__wbindgen_debug_string:n.fY,__wbindgen_throw:n.Or}});e.a(t,(t=>{var r=t([e(171)]);return r.then?r.then(n):n(r)}),1)},10:(t,r,e)=>{"use strict";e.a(t,(async t=>{e.r(r);var n=e(171),_=t([n]);(0,(n=(_.then?await _:_)[0]).QC)();const a=document.getElementById("game-canvas");a.addEventListener("click",(t=>{var r=function(t,r){var e=t.getBoundingClientRect();return{x:r.clientX-e.left,y:r.clientY-e.top}}(a,t),e={x:Math.floor(r.x/480*8),y:Math.floor(r.y/480*8)};console.log(e)}));let i=new n.Ck(480,480,"game-canvas"),o=new n.lA(8,8,3,n.SZ.Black,"game-canvas",480,480);o.set_painter(i),o.draw()}))}}]);
//# sourceMappingURL=10.bootstrap.js.map