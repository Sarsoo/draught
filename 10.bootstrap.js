"use strict";(self.webpackChunkdraught=self.webpackChunkdraught||[]).push([[10],{171:(t,e,r)=>{r.a(t,(async(n,_)=>{try{r.d(e,{C4:()=>V,Ck:()=>q,DA:()=>G,EF:()=>Tt,HS:()=>Q,Ih:()=>W,In:()=>ot,Ip:()=>xt,KK:()=>pt,KQ:()=>yt,M9:()=>it,OF:()=>wt,Oo:()=>It,Or:()=>Nt,Os:()=>$,SZ:()=>S,T8:()=>_t,TL:()=>ft,Tt:()=>N,U5:()=>L,U7:()=>at,Ws:()=>nt,XP:()=>Z,YN:()=>Ot,_3:()=>ht,ae:()=>F,bx:()=>bt,cb:()=>kt,fO:()=>X,fY:()=>jt,fi:()=>gt,gk:()=>K,h4:()=>H,hx:()=>ut,i0:()=>j,iY:()=>Y,k4:()=>dt,kq:()=>et,lA:()=>A,mS:()=>J,m_:()=>St,nf:()=>rt,oH:()=>Bt,pt:()=>tt,pv:()=>ct,r2:()=>z,rC:()=>Et,rh:()=>U,u6:()=>st,ug:()=>M,vm:()=>vt,wc:()=>E,xB:()=>mt,yX:()=>P,yq:()=>D,z2:()=>lt});var a=r(657);t=r.hmd(t);var c=n([a]);a=(c.then?(await c)():c)[0];const o=new Array(32).fill(void 0);function i(t){return o[t]}o.push(void 0,null,!0,!1);let s=o.length;function u(t){t<36||(o[t]=s,s=t)}function l(t){const e=i(t);return u(t),e}let p=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});p.decode();let d=null;function b(){return null!==d&&d.buffer===a.memory.buffer||(d=new Uint8Array(a.memory.buffer)),d}function g(t,e){return p.decode(b().subarray(t,t+e))}function f(t){s===o.length&&o.push(o.length+1);const e=s;return s=o[e],o[e]=t,e}function h(t){const e=typeof t;if("number"==e||"boolean"==e||null==t)return`${t}`;if("string"==e)return`"${t}"`;if("symbol"==e){const e=t.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=t.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(t)){const e=t.length;let r="[";e>0&&(r+=h(t[0]));for(let n=1;n<e;n++)r+=", "+h(t[n]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(t));let n;if(!(r.length>1))return toString.call(t);if(n=r[1],"Object"==n)try{return"Object("+JSON.stringify(t)+")"}catch(t){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:n}let w=0,m=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof m.encodeInto?function(t,e){return m.encodeInto(t,e)}:function(t,e){const r=m.encode(t);return e.set(r),{read:t.length,written:r.length}};function v(t,e,r){if(void 0===r){const r=m.encode(t),n=e(r.length);return b().subarray(n,n+r.length).set(r),w=r.length,n}let n=t.length,_=e(n);const a=b();let c=0;for(;c<n;c++){const e=t.charCodeAt(c);if(e>127)break;a[_+c]=e}if(c!==n){0!==c&&(t=t.slice(c)),_=r(_,n,n=c+3*t.length);const e=b().subarray(_+c,_+n);c+=y(t,e).written}return w=c,_}let T=null;function k(){return null!==T&&T.buffer===a.memory.buffer||(T=new Int32Array(a.memory.buffer)),T}function x(t){return null==t}function I(t,e){if(!(t instanceof e))throw new Error(`expected instance of ${e.name}`);return t.ptr}function E(){a.init_wasm()}function O(t,e){try{return t.apply(this,e)}catch(t){a.__wbindgen_exn_store(f(t))}}Object.freeze({Move:0,0:"Move",Jump:1,1:"Jump"});const S=Object.freeze({Black:0,0:"Black",White:1,1:"White"}),j=(Object.freeze({Man:0,0:"Man",King:1,1:"King"}),Object.freeze({Empty:0,0:"Empty",Occupied:1,1:"Occupied",Unplayable:2,2:"Unplayable"})),N=Object.freeze({Allowed:0,0:"Allowed",UnoccupiedSrc:1,1:"UnoccupiedSrc",OccupiedDest:2,2:"OccupiedDest",OutOfBounds:3,3:"OutOfBounds",Unplayable:4,4:"Unplayable",WrongTeamSrc:5,5:"WrongTeamSrc",IllegalTrajectory:6,6:"IllegalTrajectory",NoJumpablePiece:7,7:"NoJumpablePiece",JumpingSameTeam:8,8:"JumpingSameTeam"});class B{static __wrap(t){const e=Object.create(B.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();a.__wbg_board_free(t)}get width(){return a.__wbg_get_board_width(this.ptr)>>>0}set width(t){a.__wbg_set_board_width(this.ptr,t)}get height(){return a.__wbg_get_board_height(this.ptr)>>>0}set height(t){a.__wbg_set_board_height(this.ptr,t)}get current_turn(){return a.__wbg_get_board_current_turn(this.ptr)>>>0}set current_turn(t){a.__wbg_set_board_current_turn(this.ptr,t)}cell(t){const e=a.board_cell(this.ptr,t);return R.__wrap(e)}set_cell(t,e){I(e,R);var r=e.ptr;e.ptr=0,a.board_set_cell(this.ptr,t,r)}grid_cell(t){I(t,U);var e=t.ptr;t.ptr=0;const r=a.board_grid_cell(this.ptr,e);return R.__wrap(r)}cell_index(t,e){return a.board_cell_index(this.ptr,t,e)>>>0}cell_idx(t){I(t,U);var e=t.ptr;return t.ptr=0,a.board_cell_idx(this.ptr,e)>>>0}board_index(t){const e=a.board_board_index(this.ptr,t);return U.__wrap(e)}can_move(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;return e.ptr=0,a.board_can_move(this.ptr,r,n)>>>0}validate_man_move(t,e,r){I(t,U);var n=t.ptr;t.ptr=0,I(e,U);var _=e.ptr;e.ptr=0,I(r,C);var c=r.ptr;return r.ptr=0,a.board_validate_man_move(this.ptr,n,_,c)>>>0}validate_king_move(t,e,r){I(t,U);var n=t.ptr;t.ptr=0,I(e,U);var _=e.ptr;e.ptr=0,I(r,C);var c=r.ptr;return r.ptr=0,a.board_validate_king_move(this.ptr,n,_,c)>>>0}jumpee_idx(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;return e.ptr=0,a.board_jumpee_idx(this.ptr,r,n)>>>0}cells(){return a.board_cells(this.ptr)}num_cells(){return a.board_num_cells(this.ptr)>>>0}num_pieces(){return a.board_num_pieces(this.ptr)>>>0}num_player(t){return a.board_num_player(this.ptr,t)>>>0}score(){return a.board_score(this.ptr)}cell_state(t){return a.board_cell_state(this.ptr,t)>>>0}apply_move(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;e.ptr=0;const _=a.board_apply_move(this.ptr,r,n);return B.__wrap(_)}apply_jump(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;e.ptr=0;const _=a.board_apply_jump(this.ptr,r,n);return B.__wrap(_)}king_row_idx(){return a.board_king_row_idx(this.ptr)>>>0}static validate_jumpee(t,e){I(t,R);var r=t.ptr;t.ptr=0,I(e,C);var n=e.ptr;return e.ptr=0,a.board_validate_jumpee(r,n)>>>0}static check_jumpee_team(t,e){I(t,C);var r=t.ptr;t.ptr=0,I(e,C);var n=e.ptr;return e.ptr=0,0!==a.board_check_jumpee_team(r,n)}constructor(t,e,r){const n=a.board_new(t,e,r);return B.__wrap(n)}static init_game(t,e){I(t,B);var r=t.ptr;t.ptr=0;const n=a.board_init_game(r,e);return B.__wrap(n)}}class U{static __wrap(t){const e=Object.create(U.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();a.__wbg_brdidx_free(t)}get row(){return a.__wbg_get_brdidx_row(this.ptr)>>>0}set row(t){a.__wbg_set_brdidx_row(this.ptr,t)}get col(){return a.__wbg_get_brdidx_col(this.ptr)>>>0}set col(t){a.__wbg_set_brdidx_col(this.ptr,t)}constructor(t,e){const r=a.brdidx_from(t,e);return U.__wrap(r)}eq(t){return I(t,U),0!==a.brdidx_eq(this.ptr,t.ptr)}}class A{static __wrap(t){const e=Object.create(A.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();a.__wbg_game_free(t)}get last_node_count(){return a.__wbg_get_game_last_node_count(this.ptr)>>>0}set last_node_count(t){a.__wbg_set_game_last_node_count(this.ptr,t)}get perfect_chance(){return a.__wbg_get_game_perfect_chance(this.ptr)}set perfect_chance(t){a.__wbg_set_game_perfect_chance(this.ptr,t)}current_board_cells(){return a.game_current_board_cells(this.ptr)}current_board_len(){return a.game_current_board_len(this.ptr)>>>0}current_turn(){return a.game_current_turn(this.ptr)>>>0}score(){return a.game_score(this.ptr)}winning(){const t=a.game_winning(this.ptr);return 2===t?void 0:t}has_won(){const t=a.game_has_won(this.ptr);return 2===t?void 0:t}current_cell_state(t){I(t,U);const e=a.game_current_cell_state(this.ptr,t.ptr);return R.__wrap(e)}set_search_depth(t){a.game_set_search_depth(this.ptr,t)}set_selected(t){I(t,U),a.game_set_selected(this.ptr,t.ptr)}set_perfect_chance(t){a.game_set_perfect_chance(this.ptr,t)}clear_selected(){a.game_clear_selected(this.ptr)}make_move(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;return e.ptr=0,a.game_make_move(this.ptr,r,n)>>>0}execute_move(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;e.ptr=0,a.game_execute_move(this.ptr,r,n)}execute_jump(t,e){I(t,U);var r=t.ptr;t.ptr=0,I(e,U);var n=e.ptr;e.ptr=0,a.game_execute_jump(this.ptr,r,n)}push_new_board(t){I(t,B);var e=t.ptr;t.ptr=0,a.game_push_new_board(this.ptr,e)}set_current(t){I(t,B);var e=t.ptr;t.ptr=0,a.game_set_current(this.ptr,e)}constructor(t,e,r,n,_){const c=a.game_new(t,e,r,n,_);return A.__wrap(c)}static new_with_canvas(t,e,r,n,_,c,o,i){const s=v(c,a.__wbindgen_malloc,a.__wbindgen_realloc),u=w,l=a.game_new_with_canvas(t,e,r,n,_,s,u,o,i);return A.__wrap(l)}set_painter(t){I(t,q);var e=t.ptr;t.ptr=0,a.game_set_painter(this.ptr,e)}draw(){a.game_draw(this.ptr)}ai_move(){a.game_ai_move(this.ptr)}}class q{static __wrap(t){const e=Object.create(q.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();a.__wbg_painter_free(t)}constructor(t,e,r){const n=v(r,a.__wbindgen_malloc,a.__wbindgen_realloc),_=w,c=a.painter_new(t,e,n,_);return q.__wrap(c)}static new_with_canvas(t,e,r){const n=a.painter_new_with_canvas(t,e,f(r));return q.__wrap(n)}set_square_outline(t){a.painter_set_square_outline(this.ptr,f(t))}set_outline_width(t){a.painter_set_outline_width(this.ptr,t)}set_draw_outline(t){a.painter_set_draw_outline(this.ptr,t)}reset_dimensions(){a.painter_reset_dimensions(this.ptr)}validate_board_dim(t){return I(t,B),0!==a.painter_validate_board_dim(this.ptr,t.ptr)}draw(t){I(t,B),a.painter_draw(this.ptr,t.ptr)}}class C{static __wrap(t){const e=Object.create(C.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();a.__wbg_piece_free(t)}get team(){return a.__wbg_get_piece_team(this.ptr)>>>0}set team(t){a.__wbg_set_piece_team(this.ptr,t)}get strength(){return a.__wbg_get_piece_strength(this.ptr)>>>0}set strength(t){a.__wbg_set_piece_strength(this.ptr,t)}constructor(t,e){const r=a.piece_new(t,e);return C.__wrap(r)}}class R{static __wrap(t){const e=Object.create(R.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();a.__wbg_square_free(t)}get occupant(){const t=a.__wbg_get_square_occupant(this.ptr);return 0===t?void 0:C.__wrap(t)}set occupant(t){let e=0;x(t)||(I(t,C),e=t.ptr,t.ptr=0),a.__wbg_set_square_occupant(this.ptr,e)}get state(){return a.__wbg_get_square_state(this.ptr)>>>0}set state(t){a.__wbg_set_square_state(this.ptr,t)}constructor(t,e){let r=0;x(e)||(I(e,C),r=e.ptr,e.ptr=0);const n=a.square_new(t,r);return R.__wrap(n)}static pc(t,e){const r=a.square_pc(t,e);return R.__wrap(r)}static empty(){const t=a.square_empty();return R.__wrap(t)}static unplay(){const t=a.square_unplay();return R.__wrap(t)}}function M(t){l(t)}function H(t,e){return f(g(t,e))}function W(){return f(new Error)}function D(t,e){const r=v(i(e).stack,a.__wbindgen_malloc,a.__wbindgen_realloc),n=w;k()[t/4+1]=n,k()[t/4+0]=r}function K(t,e){try{console.error(g(t,e))}finally{a.__wbindgen_free(t,e)}}function $(t,e,r){var n,_;i(t).randomFillSync((n=e,_=r,b().subarray(n/1,n/1+_)))}function F(t,e){i(t).getRandomValues(i(e))}function L(){return O((function(){return f(self.self)}),arguments)}function Y(t){return f(i(t).crypto)}function J(t){return f(i(t).msCrypto)}function Z(t){return void 0===i(t)}function z(t,e,r){return f(i(t).require(g(e,r)))}function P(t){return f(i(t).getRandomValues)}function G(){return f(t)}function X(t){return i(t)instanceof Window}function V(t){const e=i(t).document;return x(e)?0:f(e)}function Q(t,e,r){const n=i(t).getElementById(g(e,r));return x(n)?0:f(n)}function tt(t){console.error(i(t))}function et(t){console.log(i(t))}function rt(t){return i(t)instanceof CanvasRenderingContext2D}function nt(t,e){i(t).strokeStyle=i(e)}function _t(t,e){i(t).fillStyle=i(e)}function at(t,e){i(t).lineWidth=e}function ct(t){i(t).beginPath()}function ot(t){i(t).fill()}function it(t){i(t).stroke()}function st(){return O((function(t,e,r,n,_,a){i(t).arc(e,r,n,_,a)}),arguments)}function ut(t,e,r,n,_){i(t).fillRect(e,r,n,_)}function lt(t,e,r,n,_){i(t).strokeRect(e,r,n,_)}function pt(t){return i(t)instanceof HTMLCanvasElement}function dt(t,e){i(t).width=e>>>0}function bt(t,e){i(t).height=e>>>0}function gt(){return O((function(t,e,r){const n=i(t).getContext(g(e,r));return x(n)?0:f(n)}),arguments)}function ft(t,e){return f(new Function(g(t,e)))}function ht(){return O((function(t,e){return f(i(t).call(i(e)))}),arguments)}function wt(){return O((function(){return f(self.self)}),arguments)}function mt(){return O((function(){return f(window.window)}),arguments)}function yt(){return O((function(){return f(globalThis.globalThis)}),arguments)}function vt(){return O((function(){return f(r.g.global)}),arguments)}function Tt(t){return f(i(t).buffer)}function kt(t){return f(new Uint8Array(i(t)))}function xt(t,e,r){i(t).set(i(e),r>>>0)}function It(t){return i(t).length}function Et(t){return f(new Uint8Array(t>>>0))}function Ot(t,e,r){return f(i(t).subarray(e>>>0,r>>>0))}function St(t){return f(i(t))}function jt(t,e){const r=v(h(i(e)),a.__wbindgen_malloc,a.__wbindgen_realloc),n=w;k()[t/4+1]=n,k()[t/4+0]=r}function Nt(t,e){throw new Error(g(t,e))}function Bt(){return f(a.memory)}_()}catch(Ut){_(Ut)}}))},10:(t,e,r)=>{r.a(t,(async(t,n)=>{try{r.r(e);var _=r(171),a=t([_]);_=(a.then?(await a)():a)[0];const l=720,p=720;var c=8,o=8,i=3,s=4,u=.5;const d=3e3,b=3e3,g={HUMAN_TURN:{THINKING:"human_turn.thinking",FROM_SELECTED:"human_turn.from_selected"},AI_TURN:"ai_turn"};(0,_.wc)(),document.getElementById("status-p");const f=document.getElementById("status-d"),h=document.getElementById("team-p"),w=document.getElementById("node-count"),m=document.getElementById("winning-p");document.getElementById("startBtn").onclick=S;let y=null,v=null,T=B,k=g.HUMAN_TURN.THINKING,x=null,I=null,E=[];S();const O=document.getElementById("game-canvas");function S(){x=new _.lA(c,o,i,_.SZ.Black,s),I=new _.Ck(l,p,"game-canvas"),x.set_painter(I),x.draw(),clearInterval(y),U(),A(),E=[],k=g.HUMAN_TURN.THINKING}function j(t){switch(k){case g.HUMAN_TURN.THINKING:if(x.current_cell_state(t).state!=_.i0.Occupied)return;if(x.current_cell_state(t).occupant.team!=x.current_turn())return;E.push(t),k=g.HUMAN_TURN.FROM_SELECTED,x.set_selected(t),x.draw();break;case g.HUMAN_TURN.FROM_SELECTED:if(!E[0].eq(t)){if(x.current_cell_state(t).state!=_.i0.Empty)return;if(E.push(t),2!=E.length)return T(`Error: wrong number of clicks to process ${E.length}`),void console.error(`Error: wrong number of clicks to process ${E.length}`);switch(x.make_move(E[0],E[1])){case _.Tt.Allowed:if(F.checked&&void 0===x.has_won()){let t=performance.now();x.ai_move();let e=performance.now();w.innerText=`searched ${x.last_node_count.toLocaleString("en-GB")} possible moves in ${(e-t).toLocaleString()}ms`}break;case _.Tt.IllegalTrajectory:T("You can't move like that!");break;case _.Tt.JumpingSameTeam:T("You can't jump your own piece!");break;case _.Tt.NoJumpablePiece:T("There's nothing to jump!");break;case _.Tt.OccupiedDest:T("There's a piece there!");break;case _.Tt.OutOfBounds:T("That square's not on the board! (how have you managed that?)");break;case _.Tt.UnoccupiedSrc:T("There's no piece to move!");break;case _.Tt.Unplayable:T("That's not a playable square!");break;case _.Tt.WrongTeamSrc:T("That's not your piece!")}}x.clear_selected(),x.draw(),E=[],k=g.HUMAN_TURN.THINKING;break;case g.AI_TURN:console.log("It's the AI's turn!")}U(),A(),q()}function N(t,e){var r=t.getBoundingClientRect();return{x:e.clientX-r.left,y:e.clientY-r.top}}function B(t,e="danger",r=!0){null!=v&&clearInterval(v),f.className=`alert alert-${e}`,f.innerText=t,f.hidden=!1,r&&(v=setTimeout((()=>{f.hidden=!0}),d))}function U(){switch(x.current_turn()){case _.SZ.White:h.innerText="⚪ White ⚪";break;case _.SZ.Black:h.innerText="🔴 Black 🔴"}}function A(){switch(x.winning()){case void 0:m.innerText="";break;case _.SZ.White:m.innerText="👑 White 👑";break;case _.SZ.Black:m.innerText="👑 Black 👑"}}function q(){switch(x.has_won()){case void 0:break;case _.SZ.White:T("You Lost!"),y=setInterval((()=>{S()}),b);break;case _.SZ.Black:T("You Won!","success"),y=setInterval((()=>{S()}),b)}}O.addEventListener("click",(t=>{var e=N(O,t);j(new _.rh(Math.floor(e.y/O.clientHeight*o),Math.floor(e.x/O.clientWidth*c)))}));const C=document.getElementById("width"),R=()=>{c=parseInt(C.value),S()};C.onchange=R,C.value=8;const M=document.getElementById("height"),H=()=>{o=parseInt(M.value),W.max=o/2-1,S()};M.onchange=H,M.value=8;const W=document.getElementById("play_rows"),D=()=>{i=parseInt(W.value),S()};W.onchange=D,W.value=3;const K=document.getElementById("ai_search_depth"),$=()=>{s=parseInt(K.value),x.set_search_depth(s),s>4&&T("This increases thinking time exponentially, be careful (probably don't go past 6)","warning")};K.onchange=$,K.value=4;const F=document.getElementById("ai-checkbox"),L=()=>{};F.onchange=L;const Y=document.getElementById("ai_difficulty"),J=()=>{u=parseInt(Y.value)/100,x.set_perfect_chance(u)};Y.onchange=J,Y.value=50,n()}catch(Z){n(Z)}}))},657:(t,e,r)=>{r.a(t,(async(n,_)=>{try{var a,c=n([a=r(171)]),[a]=c.then?(await c)():c;await r.v(e,t.id,"3779a77aa968ad91d198",{"./draught_bg.js":{__wbindgen_object_drop_ref:a.ug,__wbindgen_string_new:a.h4,__wbg_new_693216e109162396:a.Ih,__wbg_stack_0ddaca5d1abfb52f:a.yq,__wbg_error_09919627ac0992f5:a.gk,__wbg_randomFillSync_d2ba53160aec6aba:a.Os,__wbg_getRandomValues_e57c9b75ddead065:a.ae,__wbg_self_86b4b13392c7af56:a.U5,__wbg_crypto_b8c92eaac23d0d80:a.iY,__wbg_msCrypto_9ad6677321a08dd8:a.mS,__wbindgen_is_undefined:a.XP,__wbg_require_f5521a5b85ad2542:a.r2,__wbg_getRandomValues_dd27e6b0652b3236:a.yX,__wbg_static_accessor_MODULE_452b4680e8614c81:a.DA,__wbg_instanceof_Window_0e6c0f1096d66c3c:a.fO,__wbg_document_99eddbbc11ec831e:a.C4,__wbg_getElementById_f83c5de20dc455d6:a.HS,__wbg_error_8ff19d586a987aef:a.pt,__wbg_log_e8ba7b992c7ad0eb:a.kq,__wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f:a.nf,__wbg_setstrokeStyle_32540003cbfe210b:a.Ws,__wbg_setfillStyle_1d391c4891a6ec4d:a.T8,__wbg_setlineWidth_6f1b76036ab98bfc:a.U7,__wbg_beginPath_e040b5521d41f537:a.pv,__wbg_fill_b6e37fbbefb55ae0:a.In,__wbg_stroke_63664360a52ce7d1:a.M9,__wbg_arc_85205a36bd04df0a:a.u6,__wbg_fillRect_59b38b7e6f8d0717:a.hx,__wbg_strokeRect_469c3838c9d01537:a.z2,__wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef:a.KK,__wbg_setwidth_654d8adcd4979eed:a.k4,__wbg_setheight_2b662384bfacb65c:a.bx,__wbg_getContext_0c19ba5c037e057f:a.fi,__wbg_newnoargs_e23b458e372830de:a.TL,__wbg_call_ae78342adc33730a:a._3,__wbg_self_99737b4dcdf6f0d8:a.OF,__wbg_window_9b61fbbf3564c4fb:a.xB,__wbg_globalThis_8e275ef40caea3a3:a.KQ,__wbg_global_5de1e0f82bddcd27:a.vm,__wbg_buffer_7af23f65f6c64548:a.EF,__wbg_new_cc9018bd6f283b6f:a.cb,__wbg_set_f25e869e4565d2a2:a.Ip,__wbg_length_0acb1cf9bbaf8519:a.Oo,__wbg_newwithlength_8f0657faca9f1422:a.rC,__wbg_subarray_da527dbd24eafb6b:a.YN,__wbindgen_object_clone_ref:a.m_,__wbindgen_debug_string:a.fY,__wbindgen_throw:a.Or,__wbindgen_memory:a.oH}}),_()}catch(t){_(t)}}),1)}}]);
//# sourceMappingURL=10.bootstrap.js.map