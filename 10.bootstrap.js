(self.webpackChunkdraught=self.webpackChunkdraught||[]).push([[10],{171:(t,e,r)=>{"use strict";r.a(t,(async n=>{r.d(e,{wc:()=>x,SZ:()=>j,i0:()=>O,Tt:()=>E,rh:()=>q,lA:()=>B,Ck:()=>I,ug:()=>C,h4:()=>U,h9:()=>$,Dz:()=>M,kF:()=>D,s8:()=>J,WB:()=>P,cP:()=>R,a:()=>z,l_:()=>F,U_:()=>N,EN:()=>Y,qu:()=>K,Wg:()=>L,B3:()=>Q,I6:()=>Z,Qb:()=>X,cI:()=>H,QK:()=>G,Md:()=>V,ox:()=>tt,ES:()=>et,Ae:()=>rt,UL:()=>nt,qw:()=>_t,tS:()=>at,R$:()=>ct,md:()=>it,IF:()=>ot,XP:()=>st,m_:()=>ut,fY:()=>pt,Or:()=>lt});var _=r(813);t=r.hmd(t);var a=n([_]);_=(a.then?await a:a)[0];const c=new Array(32).fill(void 0);function i(t){return c[t]}c.push(void 0,null,!0,!1);let o=c.length;let s=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let u=null;function p(){return null!==u&&u.buffer===_.memory.buffer||(u=new Uint8Array(_.memory.buffer)),u}function l(t,e){return s.decode(p().subarray(t,t+e))}function d(t){o===c.length&&c.push(c.length+1);const e=o;return o=c[e],c[e]=t,e}function b(t){const e=typeof t;if("number"==e||"boolean"==e||null==t)return`${t}`;if("string"==e)return`"${t}"`;if("symbol"==e){const e=t.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=t.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(t)){const e=t.length;let r="[";e>0&&(r+=b(t[0]));for(let n=1;n<e;n++)r+=", "+b(t[n]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(t));let n;if(!(r.length>1))return toString.call(t);if(n=r[1],"Object"==n)try{return"Object("+JSON.stringify(t)+")"}catch(t){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:n}let g=0,w=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof w.encodeInto?function(t,e){return w.encodeInto(t,e)}:function(t,e){const r=w.encode(t);return e.set(r),{read:t.length,written:r.length}};function f(t,e,r){if(void 0===r){const r=w.encode(t),n=e(r.length);return p().subarray(n,n+r.length).set(r),g=r.length,n}let n=t.length,_=e(n);const a=p();let c=0;for(;c<n;c++){const e=t.charCodeAt(c);if(e>127)break;a[_+c]=e}if(c!==n){0!==c&&(t=t.slice(c)),_=r(_,n,n=c+3*t.length);const e=p().subarray(_+c,_+n);c+=h(t,e).written}return g=c,_}let m=null;function v(){return null!==m&&m.buffer===_.memory.buffer||(m=new Int32Array(_.memory.buffer)),m}function y(t){return null==t}function k(t,e){if(!(t instanceof e))throw new Error(`expected instance of ${e.name}`);return t.ptr}function x(){_.init_wasm()}function T(t,e){try{return t.apply(this,e)}catch(t){_.__wbindgen_exn_store(d(t))}}Object.freeze({Move:0,0:"Move",Jump:1,1:"Jump"});const j=Object.freeze({Black:0,0:"Black",White:1,1:"White"}),O=(Object.freeze({Man:0,0:"Man",King:1,1:"King"}),Object.freeze({Empty:0,0:"Empty",Occupied:1,1:"Occupied",Unplayable:2,2:"Unplayable"})),E=Object.freeze({Allowed:0,0:"Allowed",UnoccupiedSrc:1,1:"UnoccupiedSrc",OccupiedDest:2,2:"OccupiedDest",OutOfBounds:3,3:"OutOfBounds",Unplayable:4,4:"Unplayable",WrongTeamSrc:5,5:"WrongTeamSrc",IllegalTrajectory:6,6:"IllegalTrajectory",NoJumpablePiece:7,7:"NoJumpablePiece",JumpingSameTeam:8,8:"JumpingSameTeam"});class S{static __wrap(t){const e=Object.create(S.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_board_free(t)}get width(){return _.__wbg_get_board_width(this.ptr)>>>0}set width(t){_.__wbg_set_board_width(this.ptr,t)}get height(){return _.__wbg_get_board_height(this.ptr)>>>0}set height(t){_.__wbg_set_board_height(this.ptr,t)}get current_turn(){return _.__wbg_get_board_current_turn(this.ptr)>>>0}set current_turn(t){_.__wbg_set_board_current_turn(this.ptr,t)}cell(t){var e=_.board_cell(this.ptr,t);return A.__wrap(e)}set_cell(t,e){k(e,A);var r=e.ptr;e.ptr=0,_.board_set_cell(this.ptr,t,r)}grid_cell(t){k(t,q);var e=t.ptr;t.ptr=0;var r=_.board_grid_cell(this.ptr,e);return A.__wrap(r)}cell_index(t,e){return _.board_cell_index(this.ptr,t,e)>>>0}cell_idx(t){k(t,q);var e=t.ptr;return t.ptr=0,_.board_cell_idx(this.ptr,e)>>>0}board_index(t){var e=_.board_board_index(this.ptr,t);return q.__wrap(e)}can_move(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;return e.ptr=0,_.board_can_move(this.ptr,r,n)>>>0}validate_man_move(t,e,r){k(t,q);var n=t.ptr;t.ptr=0,k(e,q);var a=e.ptr;e.ptr=0,k(r,W);var c=r.ptr;return r.ptr=0,_.board_validate_man_move(this.ptr,n,a,c)>>>0}validate_king_move(t,e,r){k(t,q);var n=t.ptr;t.ptr=0,k(e,q);var a=e.ptr;e.ptr=0,k(r,W);var c=r.ptr;return r.ptr=0,_.board_validate_king_move(this.ptr,n,a,c)>>>0}jumpee_idx(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;return e.ptr=0,_.board_jumpee_idx(this.ptr,r,n)>>>0}cells(){return _.board_cells(this.ptr)}num_cells(){return _.board_num_cells(this.ptr)>>>0}num_pieces(){return _.board_num_pieces(this.ptr)>>>0}num_player(t){return _.board_num_player(this.ptr,t)>>>0}score(){return _.board_score(this.ptr)}cell_state(t){return _.board_cell_state(this.ptr,t)>>>0}apply_move(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;e.ptr=0;var a=_.board_apply_move(this.ptr,r,n);return S.__wrap(a)}apply_jump(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;e.ptr=0;var a=_.board_apply_jump(this.ptr,r,n);return S.__wrap(a)}king_row_idx(){return _.board_king_row_idx(this.ptr)>>>0}static validate_jumpee(t,e){k(t,A);var r=t.ptr;t.ptr=0,k(e,W);var n=e.ptr;return e.ptr=0,_.board_validate_jumpee(r,n)>>>0}static check_jumpee_team(t,e){k(t,W);var r=t.ptr;t.ptr=0,k(e,W);var n=e.ptr;return e.ptr=0,0!==_.board_check_jumpee_team(r,n)}constructor(t,e,r){var n=_.board_new(t,e,r);return S.__wrap(n)}static init_game(t,e){k(t,S);var r=t.ptr;t.ptr=0;var n=_.board_init_game(r,e);return S.__wrap(n)}}class q{static __wrap(t){const e=Object.create(q.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_brdidx_free(t)}get row(){return _.__wbg_get_brdidx_row(this.ptr)>>>0}set row(t){_.__wbg_set_brdidx_row(this.ptr,t)}get col(){return _.__wbg_get_brdidx_col(this.ptr)>>>0}set col(t){_.__wbg_set_brdidx_col(this.ptr,t)}constructor(t,e){var r=_.brdidx_from(t,e);return q.__wrap(r)}eq(t){return k(t,q),0!==_.brdidx_eq(this.ptr,t.ptr)}}class B{static __wrap(t){const e=Object.create(B.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_game_free(t)}current_board_cells(){return _.game_current_board_cells(this.ptr)}current_board_len(){return _.game_current_board_len(this.ptr)>>>0}current_turn(){return _.game_current_turn(this.ptr)>>>0}current_cell_state(t){k(t,q);var e=_.game_current_cell_state(this.ptr,t.ptr);return A.__wrap(e)}set_selected(t){k(t,q),_.game_set_selected(this.ptr,t.ptr)}clear_selected(){_.game_clear_selected(this.ptr)}make_move(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;return e.ptr=0,_.game_make_move(this.ptr,r,n)>>>0}execute_move(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;e.ptr=0,_.game_execute_move(this.ptr,r,n)}execute_jump(t,e){k(t,q);var r=t.ptr;t.ptr=0,k(e,q);var n=e.ptr;e.ptr=0,_.game_execute_jump(this.ptr,r,n)}push_new_board(t){k(t,S);var e=t.ptr;t.ptr=0,_.game_push_new_board(this.ptr,e)}set_current(t){k(t,S);var e=t.ptr;t.ptr=0,_.game_set_current(this.ptr,e)}constructor(t,e,r,n){var a=_.game_new(t,e,r,n);return B.__wrap(a)}static new_with_canvas(t,e,r,n,a,c,i){var o=f(a,_.__wbindgen_malloc,_.__wbindgen_realloc),s=g,u=_.game_new_with_canvas(t,e,r,n,o,s,c,i);return B.__wrap(u)}set_painter(t){k(t,I);var e=t.ptr;t.ptr=0,_.game_set_painter(this.ptr,e)}draw(){_.game_draw(this.ptr)}}class I{static __wrap(t){const e=Object.create(I.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_painter_free(t)}constructor(t,e,r){var n=f(r,_.__wbindgen_malloc,_.__wbindgen_realloc),a=g,c=_.painter_new(t,e,n,a);return I.__wrap(c)}static new_with_canvas(t,e,r){var n=_.painter_new_with_canvas(t,e,d(r));return I.__wrap(n)}set_square_outline(t){_.painter_set_square_outline(this.ptr,d(t))}set_outline_width(t){_.painter_set_outline_width(this.ptr,t)}set_draw_outline(t){_.painter_set_draw_outline(this.ptr,t)}reset_dimensions(){_.painter_reset_dimensions(this.ptr)}validate_board_dim(t){return k(t,S),0!==_.painter_validate_board_dim(this.ptr,t.ptr)}draw(t){k(t,S),_.painter_draw(this.ptr,t.ptr)}}class W{static __wrap(t){const e=Object.create(W.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_piece_free(t)}get team(){return _.__wbg_get_piece_team(this.ptr)>>>0}set team(t){_.__wbg_set_piece_team(this.ptr,t)}get strength(){return _.__wbg_get_piece_strength(this.ptr)>>>0}set strength(t){_.__wbg_set_piece_strength(this.ptr,t)}constructor(t,e){var r=_.piece_new(t,e);return W.__wrap(r)}}class A{static __wrap(t){const e=Object.create(A.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_square_free(t)}get occupant(){var t=_.__wbg_get_square_occupant(this.ptr);return 0===t?void 0:W.__wrap(t)}set occupant(t){let e=0;y(t)||(k(t,W),e=t.ptr,t.ptr=0),_.__wbg_set_square_occupant(this.ptr,e)}get state(){return _.__wbg_get_square_state(this.ptr)>>>0}set state(t){_.__wbg_set_square_state(this.ptr,t)}constructor(t,e){let r=0;y(e)||(k(e,W),r=e.ptr,e.ptr=0);var n=_.square_new(t,r);return A.__wrap(n)}static pc(t,e){var r=_.square_pc(t,e);return A.__wrap(r)}static empty(){var t=_.square_empty();return A.__wrap(t)}static unplay(){var t=_.square_unplay();return A.__wrap(t)}}function C(t){!function(t){const e=i(t);(function(t){t<36||(c[t]=o,o=t)})(t)}(t)}function U(t,e){return d(l(t,e))}function $(){return d(new Error)}function M(t,e){var r=f(i(e).stack,_.__wbindgen_malloc,_.__wbindgen_realloc),n=g;v()[t/4+1]=n,v()[t/4+0]=r}function D(t,e){try{console.error(l(t,e))}finally{_.__wbindgen_free(t,e)}}function J(t){return i(t)instanceof Window}function P(t){var e=i(t).document;return y(e)?0:d(e)}function R(t,e,r){var n=i(t).getElementById(l(e,r));return y(n)?0:d(n)}function z(t){console.log(i(t))}function F(t){return i(t)instanceof CanvasRenderingContext2D}function N(t,e){i(t).strokeStyle=i(e)}function Y(t,e){i(t).fillStyle=i(e)}function K(t,e){i(t).lineWidth=e}function L(t){i(t).beginPath()}function Q(t){i(t).fill()}function Z(t){i(t).stroke()}function X(){return T((function(t,e,r,n,_,a){i(t).arc(e,r,n,_,a)}),arguments)}function H(t,e,r,n,_){i(t).fillRect(e,r,n,_)}function G(t,e,r,n,_){i(t).strokeRect(e,r,n,_)}function V(t){return i(t)instanceof HTMLCanvasElement}function tt(t,e){i(t).width=e>>>0}function et(t,e){i(t).height=e>>>0}function rt(){return T((function(t,e,r){var n=i(t).getContext(l(e,r));return y(n)?0:d(n)}),arguments)}function nt(t,e){return d(new Function(l(t,e)))}function _t(){return T((function(t,e){return d(i(t).call(i(e)))}),arguments)}function at(){return T((function(){return d(self.self)}),arguments)}function ct(){return T((function(){return d(window.window)}),arguments)}function it(){return T((function(){return d(globalThis.globalThis)}),arguments)}function ot(){return T((function(){return d(r.g.global)}),arguments)}function st(t){return void 0===i(t)}function ut(t){return d(i(t))}function pt(t,e){var r=f(b(i(e)),_.__wbindgen_malloc,_.__wbindgen_realloc),n=g;v()[t/4+1]=n,v()[t/4+0]=r}function lt(t,e){throw new Error(l(t,e))}}))},813:(t,e,r)=>{"use strict";var n=([n])=>r.v(e,t.id,"cc96734576b00860ab91",{"./draught_bg.js":{__wbindgen_object_drop_ref:n.ug,__wbindgen_string_new:n.h4,__wbg_new_59cb74e423758ede:n.h9,__wbg_stack_558ba5917b466edd:n.Dz,__wbg_error_4bb6c2a97407129a:n.kF,__wbg_instanceof_Window_11e25482011fc506:n.s8,__wbg_document_5aff8cd83ef968f5:n.WB,__wbg_getElementById_b180ea4ada06a837:n.cP,__wbg_log_9a99fb1af846153b:n.a,__wbg_instanceof_CanvasRenderingContext2d_779e79c4121aa91b:n.l_,__wbg_setstrokeStyle_2939ee453716e462:n.U_,__wbg_setfillStyle_af790b5baf4d3210:n.EN,__wbg_setlineWidth_3e6b1837ae38d099:n.qu,__wbg_beginPath_2378575e37027ad3:n.Wg,__wbg_fill_558339447ed6ad43:n.B3,__wbg_stroke_c1e0313c58997dcf:n.I6,__wbg_arc_fffd87d9113dce32:n.Qb,__wbg_fillRect_46ffc8d2cef7e298:n.cI,__wbg_strokeRect_7ea34fad8a7f0fe2:n.QK,__wbg_instanceof_HtmlCanvasElement_fd3cbbe3906d7792:n.Md,__wbg_setwidth_f3c88eb520ba8d47:n.ox,__wbg_setheight_5a1abba41e35c42a:n.ES,__wbg_getContext_813df131fcbd6e91:n.Ae,__wbg_newnoargs_9fdd8f3961dd1bee:n.UL,__wbg_call_ba36642bd901572b:n.qw,__wbg_self_bb69a836a72ec6e9:n.tS,__wbg_window_3304fc4b414c9693:n.R$,__wbg_globalThis_e0d21cabc6630763:n.md,__wbg_global_8463719227271676:n.IF,__wbindgen_is_undefined:n.XP,__wbindgen_object_clone_ref:n.m_,__wbindgen_debug_string:n.fY,__wbindgen_throw:n.Or}});r.a(t,(t=>{var e=t([r(171)]);return e.then?e.then(n):n(e)}),1)},10:(t,e,r)=>{"use strict";r.a(t,(async t=>{r.r(e);var n=r(171),_=t([n]);n=(_.then?await _:_)[0];const a="human_turn.thinking",c="human_turn.from_selected";(0,n.wc)(),document.getElementById("status-p");const i=document.getElementById("status-d"),o=document.getElementById("team-p");document.getElementById("startBtn").onclick=w;let s=null,u=function(t,e="danger",r=!0){null!=s&&clearInterval(s),i.className=`alert alert-${e}`,i.innerText=t,i.hidden=!1,r&&(s=setTimeout((()=>{i.hidden=!0}),3e3))},p=a,l=null,d=null,b=[];w();const g=document.getElementById("game-canvas");function w(){l=new n.lA(8,8,3,n.SZ.Black),d=new n.Ck(480,480,"game-canvas"),l.set_painter(d),l.draw(),h(),p=a}function h(){switch(l.current_turn()){case n.SZ.White:o.innerText="⚪ White ⚪";break;case n.SZ.Black:o.innerText="🔴 Black 🔴"}}g.addEventListener("click",(t=>{var e=function(t,e){var r=t.getBoundingClientRect();return{x:e.clientX-r.left,y:e.clientY-r.top}}(g,t);!function(t){switch(p){case a:if(l.current_cell_state(t).state!=n.i0.Occupied)return;if(l.current_cell_state(t).occupant.team!=l.current_turn())return;b.push(t),p=c,l.set_selected(t),l.draw();break;case c:if(!b[0].eq(t)){if(l.current_cell_state(t).state!=n.i0.Empty)return;if(b.push(t),2!=b.length)return u(`Error: wrong number of clicks to process ${b.length}`),void console.error(`Error: wrong number of clicks to process ${b.length}`);switch(l.make_move(b[0],b[1])){case n.Tt.Allowed:break;case n.Tt.IllegalTrajectory:u("You can't move like that!");break;case n.Tt.JumpingSameTeam:u("You can't jump your own piece!");break;case n.Tt.NoJumpablePiece:u("There's nothing to jump!");break;case n.Tt.OccupiedDest:u("There's a piece there!");break;case n.Tt.OutOfBounds:u("That square's not on the board! (how have you managed that?)");break;case n.Tt.UnoccupiedSrc:u("There's no piece to move!");break;case n.Tt.Unplayable:u("That's not a playable square!");break;case n.Tt.WrongTeamSrc:u("That's not your piece!")}}l.clear_selected(),l.draw(),b=[],p=a;break;case"ai_turn":console.log("It's the AI's turn!")}h()}(new n.rh(Math.floor(e.y/480*8),Math.floor(e.x/480*8)))}))}))}}]);
//# sourceMappingURL=10.bootstrap.js.map