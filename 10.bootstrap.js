(self.webpackChunkdraught=self.webpackChunkdraught||[]).push([[10],{171:(t,e,r)=>{"use strict";r.a(t,(async n=>{r.d(e,{wc:()=>x,SZ:()=>O,i0:()=>S,Tt:()=>I,rh:()=>E,lA:()=>B,Ck:()=>q,ug:()=>U,h4:()=>W,Ih:()=>R,yq:()=>Z,gk:()=>$,Os:()=>Y,ae:()=>M,U5:()=>D,iY:()=>z,mS:()=>J,XP:()=>P,K0:()=>F,yX:()=>G,DA:()=>L,xA:()=>N,p$:()=>H,zG:()=>X,q5:()=>K,r2:()=>V,pv:()=>Q,cF:()=>tt,N_:()=>et,S2:()=>rt,on:()=>nt,CZ:()=>_t,Ox:()=>at,kP:()=>ct,GR:()=>ot,Yl:()=>it,CU:()=>st,RA:()=>ut,Si:()=>lt,c7:()=>pt,bf:()=>dt,Z4:()=>gt,tL:()=>bt,Qu:()=>ht,EB:()=>ft,Yc:()=>wt,rf:()=>mt,Ts:()=>vt,Mz:()=>yt,Zu:()=>kt,_G:()=>xt,kC:()=>Tt,m_:()=>Ot,fY:()=>St,Or:()=>It,oH:()=>jt});var _=r(813);t=r.hmd(t);var a=n([_]);_=(a.then?await a:a)[0];const c=new Array(32).fill(void 0);function o(t){return c[t]}c.push(void 0,null,!0,!1);let i=c.length;let s=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let u=null;function l(){return null!==u&&u.buffer===_.memory.buffer||(u=new Uint8Array(_.memory.buffer)),u}function p(t,e){return s.decode(l().subarray(t,t+e))}function d(t){i===c.length&&c.push(c.length+1);const e=i;return i=c[e],c[e]=t,e}function g(t){const e=typeof t;if("number"==e||"boolean"==e||null==t)return`${t}`;if("string"==e)return`"${t}"`;if("symbol"==e){const e=t.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=t.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(t)){const e=t.length;let r="[";e>0&&(r+=g(t[0]));for(let n=1;n<e;n++)r+=", "+g(t[n]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(t));let n;if(!(r.length>1))return toString.call(t);if(n=r[1],"Object"==n)try{return"Object("+JSON.stringify(t)+")"}catch(t){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:n}let b=0,h=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const f="function"==typeof h.encodeInto?function(t,e){return h.encodeInto(t,e)}:function(t,e){const r=h.encode(t);return e.set(r),{read:t.length,written:r.length}};function w(t,e,r){if(void 0===r){const r=h.encode(t),n=e(r.length);return l().subarray(n,n+r.length).set(r),b=r.length,n}let n=t.length,_=e(n);const a=l();let c=0;for(;c<n;c++){const e=t.charCodeAt(c);if(e>127)break;a[_+c]=e}if(c!==n){0!==c&&(t=t.slice(c)),_=r(_,n,n=c+3*t.length);const e=l().subarray(_+c,_+n);c+=f(t,e).written}return b=c,_}let m=null;function v(){return null!==m&&m.buffer===_.memory.buffer||(m=new Int32Array(_.memory.buffer)),m}function y(t){return null==t}function k(t,e){if(!(t instanceof e))throw new Error(`expected instance of ${e.name}`);return t.ptr}function x(){_.init_wasm()}function T(t,e){try{return t.apply(this,e)}catch(t){_.__wbindgen_exn_store(d(t))}}Object.freeze({Move:0,0:"Move",Jump:1,1:"Jump"});const O=Object.freeze({Black:0,0:"Black",White:1,1:"White"}),S=(Object.freeze({Man:0,0:"Man",King:1,1:"King"}),Object.freeze({Empty:0,0:"Empty",Occupied:1,1:"Occupied",Unplayable:2,2:"Unplayable"})),I=Object.freeze({Allowed:0,0:"Allowed",UnoccupiedSrc:1,1:"UnoccupiedSrc",OccupiedDest:2,2:"OccupiedDest",OutOfBounds:3,3:"OutOfBounds",Unplayable:4,4:"Unplayable",WrongTeamSrc:5,5:"WrongTeamSrc",IllegalTrajectory:6,6:"IllegalTrajectory",NoJumpablePiece:7,7:"NoJumpablePiece",JumpingSameTeam:8,8:"JumpingSameTeam"});class j{static __wrap(t){const e=Object.create(j.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_board_free(t)}get width(){return _.__wbg_get_board_width(this.ptr)>>>0}set width(t){_.__wbg_set_board_width(this.ptr,t)}get height(){return _.__wbg_get_board_height(this.ptr)>>>0}set height(t){_.__wbg_set_board_height(this.ptr,t)}get current_turn(){return _.__wbg_get_board_current_turn(this.ptr)>>>0}set current_turn(t){_.__wbg_set_board_current_turn(this.ptr,t)}cell(t){var e=_.board_cell(this.ptr,t);return A.__wrap(e)}set_cell(t,e){k(e,A);var r=e.ptr;e.ptr=0,_.board_set_cell(this.ptr,t,r)}grid_cell(t){k(t,E);var e=t.ptr;t.ptr=0;var r=_.board_grid_cell(this.ptr,e);return A.__wrap(r)}cell_index(t,e){return _.board_cell_index(this.ptr,t,e)>>>0}cell_idx(t){k(t,E);var e=t.ptr;return t.ptr=0,_.board_cell_idx(this.ptr,e)>>>0}board_index(t){var e=_.board_board_index(this.ptr,t);return E.__wrap(e)}can_move(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;return e.ptr=0,_.board_can_move(this.ptr,r,n)>>>0}validate_man_move(t,e,r){k(t,E);var n=t.ptr;t.ptr=0,k(e,E);var a=e.ptr;e.ptr=0,k(r,C);var c=r.ptr;return r.ptr=0,_.board_validate_man_move(this.ptr,n,a,c)>>>0}validate_king_move(t,e,r){k(t,E);var n=t.ptr;t.ptr=0,k(e,E);var a=e.ptr;e.ptr=0,k(r,C);var c=r.ptr;return r.ptr=0,_.board_validate_king_move(this.ptr,n,a,c)>>>0}jumpee_idx(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;return e.ptr=0,_.board_jumpee_idx(this.ptr,r,n)>>>0}cells(){return _.board_cells(this.ptr)}num_cells(){return _.board_num_cells(this.ptr)>>>0}num_pieces(){return _.board_num_pieces(this.ptr)>>>0}num_player(t){return _.board_num_player(this.ptr,t)>>>0}score(){return _.board_score(this.ptr)}cell_state(t){return _.board_cell_state(this.ptr,t)>>>0}apply_move(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;e.ptr=0;var a=_.board_apply_move(this.ptr,r,n);return j.__wrap(a)}apply_jump(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;e.ptr=0;var a=_.board_apply_jump(this.ptr,r,n);return j.__wrap(a)}king_row_idx(){return _.board_king_row_idx(this.ptr)>>>0}static validate_jumpee(t,e){k(t,A);var r=t.ptr;t.ptr=0,k(e,C);var n=e.ptr;return e.ptr=0,_.board_validate_jumpee(r,n)>>>0}static check_jumpee_team(t,e){k(t,C);var r=t.ptr;t.ptr=0,k(e,C);var n=e.ptr;return e.ptr=0,0!==_.board_check_jumpee_team(r,n)}constructor(t,e,r){var n=_.board_new(t,e,r);return j.__wrap(n)}static init_game(t,e){k(t,j);var r=t.ptr;t.ptr=0;var n=_.board_init_game(r,e);return j.__wrap(n)}}class E{static __wrap(t){const e=Object.create(E.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_brdidx_free(t)}get row(){return _.__wbg_get_brdidx_row(this.ptr)>>>0}set row(t){_.__wbg_set_brdidx_row(this.ptr,t)}get col(){return _.__wbg_get_brdidx_col(this.ptr)>>>0}set col(t){_.__wbg_set_brdidx_col(this.ptr,t)}constructor(t,e){var r=_.brdidx_from(t,e);return E.__wrap(r)}eq(t){return k(t,E),0!==_.brdidx_eq(this.ptr,t.ptr)}}class B{static __wrap(t){const e=Object.create(B.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_game_free(t)}get last_node_count(){return _.__wbg_get_game_last_node_count(this.ptr)>>>0}set last_node_count(t){_.__wbg_set_game_last_node_count(this.ptr,t)}get perfect_chance(){return _.__wbg_get_game_perfect_chance(this.ptr)}set perfect_chance(t){_.__wbg_set_game_perfect_chance(this.ptr,t)}current_board_cells(){return _.game_current_board_cells(this.ptr)}current_board_len(){return _.game_current_board_len(this.ptr)>>>0}current_turn(){return _.game_current_turn(this.ptr)>>>0}score(){return _.game_score(this.ptr)}winning(){var t=_.game_winning(this.ptr);return 2===t?void 0:t}has_won(){var t=_.game_has_won(this.ptr);return 2===t?void 0:t}current_cell_state(t){k(t,E);var e=_.game_current_cell_state(this.ptr,t.ptr);return A.__wrap(e)}set_search_depth(t){_.game_set_search_depth(this.ptr,t)}set_selected(t){k(t,E),_.game_set_selected(this.ptr,t.ptr)}set_perfect_chance(t){_.game_set_perfect_chance(this.ptr,t)}clear_selected(){_.game_clear_selected(this.ptr)}make_move(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;return e.ptr=0,_.game_make_move(this.ptr,r,n)>>>0}execute_move(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;e.ptr=0,_.game_execute_move(this.ptr,r,n)}execute_jump(t,e){k(t,E);var r=t.ptr;t.ptr=0,k(e,E);var n=e.ptr;e.ptr=0,_.game_execute_jump(this.ptr,r,n)}push_new_board(t){k(t,j);var e=t.ptr;t.ptr=0,_.game_push_new_board(this.ptr,e)}set_current(t){k(t,j);var e=t.ptr;t.ptr=0,_.game_set_current(this.ptr,e)}constructor(t,e,r,n,a){var c=_.game_new(t,e,r,n,a);return B.__wrap(c)}static new_with_canvas(t,e,r,n,a,c,o,i){var s=w(c,_.__wbindgen_malloc,_.__wbindgen_realloc),u=b,l=_.game_new_with_canvas(t,e,r,n,a,s,u,o,i);return B.__wrap(l)}set_painter(t){k(t,q);var e=t.ptr;t.ptr=0,_.game_set_painter(this.ptr,e)}draw(){_.game_draw(this.ptr)}ai_move(){_.game_ai_move(this.ptr)}}class q{static __wrap(t){const e=Object.create(q.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_painter_free(t)}constructor(t,e,r){var n=w(r,_.__wbindgen_malloc,_.__wbindgen_realloc),a=b,c=_.painter_new(t,e,n,a);return q.__wrap(c)}static new_with_canvas(t,e,r){var n=_.painter_new_with_canvas(t,e,d(r));return q.__wrap(n)}set_square_outline(t){_.painter_set_square_outline(this.ptr,d(t))}set_outline_width(t){_.painter_set_outline_width(this.ptr,t)}set_draw_outline(t){_.painter_set_draw_outline(this.ptr,t)}reset_dimensions(){_.painter_reset_dimensions(this.ptr)}validate_board_dim(t){return k(t,j),0!==_.painter_validate_board_dim(this.ptr,t.ptr)}draw(t){k(t,j),_.painter_draw(this.ptr,t.ptr)}}class C{static __wrap(t){const e=Object.create(C.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_piece_free(t)}get team(){return _.__wbg_get_piece_team(this.ptr)>>>0}set team(t){_.__wbg_set_piece_team(this.ptr,t)}get strength(){return _.__wbg_get_piece_strength(this.ptr)>>>0}set strength(t){_.__wbg_set_piece_strength(this.ptr,t)}constructor(t,e){var r=_.piece_new(t,e);return C.__wrap(r)}}class A{static __wrap(t){const e=Object.create(A.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();_.__wbg_square_free(t)}get occupant(){var t=_.__wbg_get_square_occupant(this.ptr);return 0===t?void 0:C.__wrap(t)}set occupant(t){let e=0;y(t)||(k(t,C),e=t.ptr,t.ptr=0),_.__wbg_set_square_occupant(this.ptr,e)}get state(){return _.__wbg_get_square_state(this.ptr)>>>0}set state(t){_.__wbg_set_square_state(this.ptr,t)}constructor(t,e){let r=0;y(e)||(k(e,C),r=e.ptr,e.ptr=0);var n=_.square_new(t,r);return A.__wrap(n)}static pc(t,e){var r=_.square_pc(t,e);return A.__wrap(r)}static empty(){var t=_.square_empty();return A.__wrap(t)}static unplay(){var t=_.square_unplay();return A.__wrap(t)}}function U(t){!function(t){const e=o(t);(function(t){t<36||(c[t]=i,i=t)})(t)}(t)}function W(t,e){return d(p(t,e))}function R(){return d(new Error)}function Z(t,e){var r=w(o(e).stack,_.__wbindgen_malloc,_.__wbindgen_realloc),n=b;v()[t/4+1]=n,v()[t/4+0]=r}function $(t,e){try{console.error(p(t,e))}finally{_.__wbindgen_free(t,e)}}function Y(t,e,r){var n,_;o(t).randomFillSync((n=e,_=r,l().subarray(n/1,n/1+_)))}function M(t,e){o(t).getRandomValues(o(e))}function D(){return T((function(){return d(self.self)}),arguments)}function z(t){return d(o(t).crypto)}function J(t){return d(o(t).msCrypto)}function P(t){return void 0===o(t)}function F(t,e,r){return d(o(t).require(p(e,r)))}function G(t){return d(o(t).getRandomValues)}function L(){return d(t)}function N(t){return o(t)instanceof Window}function H(t){var e=o(t).document;return y(e)?0:d(e)}function X(t,e,r){var n=o(t).getElementById(p(e,r));return y(n)?0:d(n)}function K(t){console.error(o(t))}function V(t){console.log(o(t))}function Q(t){return o(t)instanceof CanvasRenderingContext2D}function tt(t,e){o(t).strokeStyle=o(e)}function et(t,e){o(t).fillStyle=o(e)}function rt(t,e){o(t).lineWidth=e}function nt(t){o(t).beginPath()}function _t(t){o(t).fill()}function at(t){o(t).stroke()}function ct(){return T((function(t,e,r,n,_,a){o(t).arc(e,r,n,_,a)}),arguments)}function ot(t,e,r,n,_){o(t).fillRect(e,r,n,_)}function it(t,e,r,n,_){o(t).strokeRect(e,r,n,_)}function st(t){return o(t)instanceof HTMLCanvasElement}function ut(t,e){o(t).width=e>>>0}function lt(t,e){o(t).height=e>>>0}function pt(){return T((function(t,e,r){var n=o(t).getContext(p(e,r));return y(n)?0:d(n)}),arguments)}function dt(t,e){return d(new Function(p(t,e)))}function gt(){return T((function(t,e){return d(o(t).call(o(e)))}),arguments)}function bt(){return T((function(){return d(self.self)}),arguments)}function ht(){return T((function(){return d(window.window)}),arguments)}function ft(){return T((function(){return d(globalThis.globalThis)}),arguments)}function wt(){return T((function(){return d(r.g.global)}),arguments)}function mt(t){return d(o(t).buffer)}function vt(t){return d(new Uint8Array(o(t)))}function yt(t,e,r){o(t).set(o(e),r>>>0)}function kt(t){return o(t).length}function xt(t){return d(new Uint8Array(t>>>0))}function Tt(t,e,r){return d(o(t).subarray(e>>>0,r>>>0))}function Ot(t){return d(o(t))}function St(t,e){var r=w(g(o(e)),_.__wbindgen_malloc,_.__wbindgen_realloc),n=b;v()[t/4+1]=n,v()[t/4+0]=r}function It(t,e){throw new Error(p(t,e))}function jt(){return d(_.memory)}}))},813:(t,e,r)=>{"use strict";var n=([n])=>r.v(e,t.id,"5834d1c20fa287719232",{"./draught_bg.js":{__wbindgen_object_drop_ref:n.ug,__wbindgen_string_new:n.h4,__wbg_new_693216e109162396:n.Ih,__wbg_stack_0ddaca5d1abfb52f:n.yq,__wbg_error_09919627ac0992f5:n.gk,__wbg_randomFillSync_d2ba53160aec6aba:n.Os,__wbg_getRandomValues_e57c9b75ddead065:n.ae,__wbg_self_86b4b13392c7af56:n.U5,__wbg_crypto_b8c92eaac23d0d80:n.iY,__wbg_msCrypto_9ad6677321a08dd8:n.mS,__wbindgen_is_undefined:n.XP,__wbg_require_f5521a5b85ad2542:n.K0,__wbg_getRandomValues_dd27e6b0652b3236:n.yX,__wbg_static_accessor_MODULE_452b4680e8614c81:n.DA,__wbg_instanceof_Window_434ce1849eb4e0fc:n.xA,__wbg_document_5edd43643d1060d9:n.p$,__wbg_getElementById_b30e88aff96f66a1:n.zG,__wbg_error_ca520cb687b085a1:n.q5,__wbg_log_fbd13631356d44e4:n.r2,__wbg_instanceof_CanvasRenderingContext2d_e8b3a478a1b32d55:n.pv,__wbg_setstrokeStyle_4c9ff9ea0c0c4092:n.cF,__wbg_setfillStyle_680401a47926faa5:n.N_,__wbg_setlineWidth_4819b1bc5c54719b:n.S2,__wbg_beginPath_5a6fcb370b5b8eb3:n.on,__wbg_fill_0ced47ff22561640:n.CZ,__wbg_stroke_46c6315e5e1f35db:n.Ox,__wbg_arc_b3cb1027882c81bb:n.kP,__wbg_fillRect_7ad775936c1881ce:n.GR,__wbg_strokeRect_a3f2f571f1ad9730:n.Yl,__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638:n.CU,__wbg_setwidth_362e8db8cbadbe96:n.RA,__wbg_setheight_28f53831182cc410:n.Si,__wbg_getContext_bd4e9445094eda84:n.c7,__wbg_newnoargs_f579424187aa1717:n.bf,__wbg_call_89558c3e96703ca1:n.Z4,__wbg_self_e23d74ae45fb17d1:n.tL,__wbg_window_b4be7f48b24ac56e:n.Qu,__wbg_globalThis_d61b1f48a57191ae:n.EB,__wbg_global_e7669da72fd7f239:n.Yc,__wbg_buffer_5e74a88a1424a2e0:n.rf,__wbg_new_e3b800e570795b3c:n.Ts,__wbg_set_5b8081e9d002f0df:n.Mz,__wbg_length_30803400a8f15c59:n.Zu,__wbg_newwithlength_5f4ce114a24dfe1e:n._G,__wbg_subarray_a68f835ca2af506f:n.kC,__wbindgen_object_clone_ref:n.m_,__wbindgen_debug_string:n.fY,__wbindgen_throw:n.Or,__wbindgen_memory:n.oH}});r.a(t,(t=>{var e=t([r(171)]);return e.then?e.then(n):n(e)}),1)},10:(t,e,r)=>{"use strict";r.a(t,(async t=>{r.r(e);var n=r(171),_=t([n]);n=(_.then?await _:_)[0];var a=8,c=8,o=3,i=4,s=.5;const u="human_turn.thinking",l="human_turn.from_selected";(0,n.wc)(),document.getElementById("status-p");const p=document.getElementById("status-d"),d=document.getElementById("team-p"),g=document.getElementById("node-count"),b=document.getElementById("winning-p");document.getElementById("startBtn").onclick=T;let h=null,f=null,w=function(t,e="danger",r=!0){null!=f&&clearInterval(f),p.className=`alert alert-${e}`,p.innerText=t,p.hidden=!1,r&&(f=setTimeout((()=>{p.hidden=!0}),3e3))},m=u,v=null,y=null,k=[];T();const x=document.getElementById("game-canvas");function T(){v=new n.lA(a,c,o,n.SZ.Black,i),y=new n.Ck(720,720,"game-canvas"),v.set_painter(y),v.draw(),clearInterval(h),O(),S(),k=[],m=u}function O(){switch(v.current_turn()){case n.SZ.White:d.innerText="⚪ White ⚪";break;case n.SZ.Black:d.innerText="🔴 Black 🔴"}}function S(){switch(v.winning()){case void 0:b.innerText="";break;case n.SZ.White:b.innerText="👑 White 👑";break;case n.SZ.Black:b.innerText="👑 Black 👑"}}x.addEventListener("click",(t=>{var e=function(t,e){var r=t.getBoundingClientRect();return{x:e.clientX-r.left,y:e.clientY-r.top}}(x,t);!function(t){switch(m){case u:if(v.current_cell_state(t).state!=n.i0.Occupied)return;if(v.current_cell_state(t).occupant.team!=v.current_turn())return;k.push(t),m=l,v.set_selected(t),v.draw();break;case l:if(!k[0].eq(t)){if(v.current_cell_state(t).state!=n.i0.Empty)return;if(k.push(t),2!=k.length)return w(`Error: wrong number of clicks to process ${k.length}`),void console.error(`Error: wrong number of clicks to process ${k.length}`);switch(v.make_move(k[0],k[1])){case n.Tt.Allowed:q.checked&&void 0===v.has_won()&&(v.ai_move(),g.innerText=`searched ${v.last_node_count.toLocaleString("en-GB")} possible moves`);break;case n.Tt.IllegalTrajectory:w("You can't move like that!");break;case n.Tt.JumpingSameTeam:w("You can't jump your own piece!");break;case n.Tt.NoJumpablePiece:w("There's nothing to jump!");break;case n.Tt.OccupiedDest:w("There's a piece there!");break;case n.Tt.OutOfBounds:w("That square's not on the board! (how have you managed that?)");break;case n.Tt.UnoccupiedSrc:w("There's no piece to move!");break;case n.Tt.Unplayable:w("That's not a playable square!");break;case n.Tt.WrongTeamSrc:w("That's not your piece!")}}v.clear_selected(),v.draw(),k=[],m=u;break;case"ai_turn":console.log("It's the AI's turn!")}O(),S(),function(){switch(v.has_won()){case void 0:break;case n.SZ.White:w("You Lost!"),h=setInterval((()=>{T()}),3e3);break;case n.SZ.Black:w("You Won!","success"),h=setInterval((()=>{T()}),3e3)}}()}(new n.rh(Math.floor(e.y/x.clientHeight*c),Math.floor(e.x/x.clientWidth*a)))}));const I=document.getElementById("width");I.onchange=()=>{a=parseInt(I.value),T()},I.value=8;const j=document.getElementById("height");j.onchange=()=>{c=parseInt(j.value),E.max=c/2-1,T()},j.value=8;const E=document.getElementById("play_rows");E.onchange=()=>{o=parseInt(E.value),T()},E.value=3;const B=document.getElementById("ai_search_depth");B.onchange=()=>{i=parseInt(B.value),v.set_search_depth(i),i>4&&w("This increases thinking time exponentially, be careful (probably don't go past 6)","warning")},B.value=4;const q=document.getElementById("ai-checkbox");q.onchange=()=>{};const C=document.getElementById("ai_difficulty");C.onchange=()=>{s=parseInt(C.value)/100,v.set_perfect_chance(s)},C.value=50}))}}]);
//# sourceMappingURL=10.bootstrap.js.map