"use strict";const version="0.4.0",canvas=document.querySelector("#glcanvas"),gl=canvas.getContext("webgl");null===gl&&alert("Unable to initialize WebGL. Your browser or machine may not support it.");var wasm_memory,Module,wasm_exports,clipboard=null,plugins=[],high_dpi=!1;function assert(e,r){!1==e&&alert(r)}function acquireVertexArrayObjectExtension(e){var r=e.getExtension("OES_vertex_array_object");r?(e.createVertexArray=function(){return r.createVertexArrayOES()},e.deleteVertexArray=function(e){r.deleteVertexArrayOES(e)},e.bindVertexArray=function(e){r.bindVertexArrayOES(e)},e.isVertexArray=function(e){return r.isVertexArrayOES(e)}):alert("Unable to get OES_vertex_array_object extension")}function acquireInstancedArraysExtension(e){var r=e.getExtension("ANGLE_instanced_arrays");r&&(e.vertexAttribDivisor=function(e,t){r.vertexAttribDivisorANGLE(e,t)},e.drawArraysInstanced=function(e,t,n,a){r.drawArraysInstancedANGLE(e,t,n,a)},e.drawElementsInstanced=function(e,t,n,a,o){r.drawElementsInstancedANGLE(e,t,n,a,o)})}function acquireDisjointTimerQueryExtension(e){var r=e.getExtension("EXT_disjoint_timer_query");r&&(e.createQuery=function(){return r.createQueryEXT()},e.beginQuery=function(e,t){return r.beginQueryEXT(e,t)},e.endQuery=function(e){return r.endQueryEXT(e)},e.deleteQuery=function(e){r.deleteQueryEXT(e)},e.getQueryObject=function(e,t){return r.getQueryObjectEXT(e,t)})}canvas.focus(),canvas.requestPointerLock=canvas.requestPointerLock||canvas.mozRequestPointerLock||function(){},document.exitPointerLock=document.exitPointerLock||document.mozExitPointerLock||function(){};try{gl.getExtension("EXT_shader_texture_lod"),gl.getExtension("OES_standard_derivatives")}catch(e){console.warn(e)}function getArray(e,r,t){return new r(wasm_memory.buffer,e,t)}function UTF8ToString(e,r){let t=new Uint8Array(wasm_memory.buffer,e);for(var n=0,a=n+r,o="";!(n>=a);){var i=t[n++];if(!i)break;if(!(128&i)){o+=String.fromCharCode(i);continue}var s=63&t[n++];if((224&i)==192){o+=String.fromCharCode((31&i)<<6|s);continue}var u=63&t[n++];if((240&i)==224?i=(15&i)<<12|s<<6|u:((248&i)!=240&&console.warn("Invalid UTF-8 leading byte 0x"+i.toString(16)+" encountered when deserializing a UTF-8 string on the asm.js/wasm heap to a JS string!"),i=(7&i)<<18|s<<12|u<<6|63&t[n++]),i<65536)o+=String.fromCharCode(i);else{var l=i-65536;o+=String.fromCharCode(55296|l>>10,56320|1023&l)}}return o}function stringToUTF8(e,r,t,n){for(var a=t,o=t+n,i=0;i<e.length;++i){var s=e.charCodeAt(i);if(s>=55296&&s<=57343&&(s=65536+((1023&s)<<10)|1023&e.charCodeAt(++i)),s<=127){if(t>=o)break;r[t++]=s}else if(s<=2047){if(t+1>=o)break;r[t++]=192|s>>6,r[t++]=128|63&s}else if(s<=65535){if(t+2>=o)break;r[t++]=224|s>>12,r[t++]=128|s>>6&63,r[t++]=128|63&s}else{if(t+3>=o)break;s>=2097152&&console.warn("Invalid Unicode code point 0x"+s.toString(16)+" encountered when serializing a JS string to an UTF-8 string on the asm.js/wasm heap! (Valid unicode code points should be in range 0-0x1FFFFF)."),r[t++]=240|s>>18,r[t++]=128|s>>12&63,r[t++]=128|s>>6&63,r[t++]=128|63&s}}return t-a}acquireVertexArrayObjectExtension(gl),acquireInstancedArraysExtension(gl),acquireDisjointTimerQueryExtension(gl),null==gl.getExtension("WEBGL_depth_texture")&&alert("Cant initialize WEBGL_depth_texture extension");var FS={loaded_files:[],unique_id:0},GL={counter:1,buffers:[],mappedBuffers:{},programs:[],framebuffers:[],renderbuffers:[],textures:[],uniforms:[],shaders:[],vaos:[],timerQueries:[],contexts:{},programInfos:{},getNewId:function(e){for(var r=GL.counter++,t=e.length;t<r;t++)e[t]=null;return r},validateGLObjectID:function(e,r,t,n){0==r||(null===e[r]?console.error(t+" called with an already deleted "+n+" ID "+r+"!"):e[r]||console.error(t+" called with an invalid "+n+" ID "+r+"!"))},getSource:function(e,r,t,n){for(var a="",o=0;o<r;++o){var i=0==n?void 0:getArray(n+4*o,Uint32Array,1)[0];a+=UTF8ToString(getArray(t+4*o,Uint32Array,1)[0],i)}return a},populateUniformTable:function(e){GL.validateGLObjectID(GL.programs,e,"populateUniformTable","program");for(var r=GL.programs[e],t=GL.programInfos[e]={uniforms:{},maxUniformLength:0,maxAttributeLength:-1,maxUniformBlockNameLength:-1},n=t.uniforms,a=gl.getProgramParameter(r,35718),o=0;o<a;++o){var i=gl.getActiveUniform(r,o),s=i.name;t.maxUniformLength=Math.max(t.maxUniformLength,s.length+1),"]"==s.slice(-1)&&(s=s.slice(0,s.lastIndexOf("[")));var u=gl.getUniformLocation(r,s);if(u){var l=GL.getNewId(GL.uniforms);n[s]=[i.size,l],GL.uniforms[l]=u;for(var c=1;c<i.size;++c){var f=s+"["+c+"]";u=gl.getUniformLocation(r,f),l=GL.getNewId(GL.uniforms),GL.uniforms[l]=u}}}}};function _glGenObject(e,r,t,n,a){for(var o=0;o<e;o++){var i=gl[t](),s=i&&GL.getNewId(n);i?(i.name=s,n[s]=i):(console.error("GL_INVALID_OPERATION"),GL.recordError(1282),alert("GL_INVALID_OPERATION in "+a+": GLctx."+t+" returned null - most likely GL context is lost!")),getArray(r+4*o,Int32Array,1)[0]=s}}function _webglGet(e,r,t){if(!r){console.error("GL_INVALID_VALUE in glGet"+t+"v(name="+e+": Function called with null out pointer!"),GL.recordError(1281);return}var n=void 0;switch(e){case 36346:n=1;break;case 36344:"EM_FUNC_SIG_PARAM_I"!=t&&"EM_FUNC_SIG_PARAM_I64"!=t&&(GL.recordError(1280),err("GL_INVALID_ENUM in glGet"+t+"v(GL_SHADER_BINARY_FORMATS): Invalid parameter type!"));return;case 34814:case 36345:n=0;break;case 34466:var a=gl.getParameter(34467);n=a?a.length:0;break;case 33309:case 33307:case 33308:assert(!1,"unimplemented")}if(void 0===n){var o=gl.getParameter(e);switch(typeof o){case"number":n=o;break;case"boolean":n=o?1:0;break;case"string":GL.recordError(1280),console.error("GL_INVALID_ENUM in glGet"+t+"v("+e+") on a name which returns a string!");return;case"object":if(null===o)switch(e){case 34964:case 35725:case 34965:case 36006:case 36007:case 32873:case 34229:case 35097:case 36389:case 34068:n=0;break;default:GL.recordError(1280),console.error("GL_INVALID_ENUM in glGet"+t+"v("+e+") and it returns null!");return}else if(o instanceof Float32Array||o instanceof Uint32Array||o instanceof Int32Array||o instanceof Array){for(var i=0;i<o.length;++i)assert(!1,"unimplemented");return}else try{n=0|o.name}catch(s){GL.recordError(1280),console.error("GL_INVALID_ENUM in glGet"+t+"v: Unknown object returned from WebGL getParameter("+e+")! (error: "+s+")");return}break;default:GL.recordError(1280),console.error("GL_INVALID_ENUM in glGet"+t+"v: Native code calling glGet"+t+"v("+e+") and it returns "+o+" of type "+typeof o+"!");return}}switch(t){case"EM_FUNC_SIG_PARAM_I64":getArray(r,Int32Array,1)[0]=n;case"EM_FUNC_SIG_PARAM_I":getArray(r,Int32Array,1)[0]=n;break;case"EM_FUNC_SIG_PARAM_F":getArray(r,Float32Array,1)[0]=n;break;case"EM_FUNC_SIG_PARAM_B":getArray(r,Int8Array,1)[0]=n?1:0;break;default:throw"internal glGet error, bad type: "+t}}function resize(e,r){var t=dpi_scale(),n=e.clientWidth*t,a=e.clientHeight*t;(e.width!=n||e.height!=a)&&(e.width=n,e.height=a,void 0!=r&&r(Math.floor(n),Math.floor(a)))}function animation(){wasm_exports.frame(),window.requestAnimationFrame(animation)}const SAPP_EVENTTYPE_TOUCHES_BEGAN=10,SAPP_EVENTTYPE_TOUCHES_MOVED=11,SAPP_EVENTTYPE_TOUCHES_ENDED=12,SAPP_EVENTTYPE_TOUCHES_CANCELED=13,SAPP_MODIFIER_SHIFT=1,SAPP_MODIFIER_CTRL=2,SAPP_MODIFIER_ALT=4,SAPP_MODIFIER_SUPER=8;function into_sapp_mousebutton(e){switch(e){case 0:return 0;case 1:return 2;case 2:return 1;default:return e}}function into_sapp_keycode(e){switch(e){case"Space":return 32;case"Quote":return 222;case"Comma":return 44;case"Minus":return 45;case"Period":return 46;case"Slash":return 189;case"Digit0":return 48;case"Digit1":return 49;case"Digit2":return 50;case"Digit3":return 51;case"Digit4":return 52;case"Digit5":return 53;case"Digit6":return 54;case"Digit7":return 55;case"Digit8":return 56;case"Digit9":return 57;case"Semicolon":return 59;case"Equal":return 61;case"KeyA":return 65;case"KeyB":return 66;case"KeyC":return 67;case"KeyD":return 68;case"KeyE":return 69;case"KeyF":return 70;case"KeyG":return 71;case"KeyH":return 72;case"KeyI":return 73;case"KeyJ":return 74;case"KeyK":return 75;case"KeyL":return 76;case"KeyM":return 77;case"KeyN":return 78;case"KeyO":return 79;case"KeyP":return 80;case"KeyQ":return 81;case"KeyR":return 82;case"KeyS":return 83;case"KeyT":return 84;case"KeyU":return 85;case"KeyV":return 86;case"KeyW":return 87;case"KeyX":return 88;case"KeyY":return 89;case"KeyZ":return 90;case"BracketLeft":return 91;case"Backslash":return 92;case"BracketRight":return 93;case"Backquote":return 96;case"Escape":return 256;case"Enter":return 257;case"Tab":return 258;case"Backspace":return 259;case"Insert":return 260;case"Delete":return 261;case"ArrowRight":return 262;case"ArrowLeft":return 263;case"ArrowDown":return 264;case"ArrowUp":return 265;case"PageUp":return 266;case"PageDown":return 267;case"Home":return 268;case"End":return 269;case"CapsLock":return 280;case"ScrollLock":return 281;case"NumLock":return 282;case"PrintScreen":return 283;case"Pause":return 284;case"F1":return 290;case"F2":return 291;case"F3":return 292;case"F4":return 293;case"F5":return 294;case"F6":return 295;case"F7":return 296;case"F8":return 297;case"F9":return 298;case"F10":return 299;case"F11":return 300;case"F12":return 301;case"F13":return 302;case"F14":return 303;case"F15":return 304;case"F16":return 305;case"F17":return 306;case"F18":return 307;case"F19":return 308;case"F20":return 309;case"F21":return 310;case"F22":return 311;case"F23":return 312;case"F24":return 313;case"Numpad0":return 320;case"Numpad1":return 321;case"Numpad2":return 322;case"Numpad3":return 323;case"Numpad4":return 324;case"Numpad5":return 325;case"Numpad6":return 326;case"Numpad7":return 327;case"Numpad8":return 328;case"Numpad9":return 329;case"NumpadDecimal":return 330;case"NumpadDivide":return 331;case"NumpadMultiply":return 332;case"NumpadSubtract":return 333;case"NumpadAdd":return 334;case"NumpadEnter":return 335;case"NumpadEqual":return 336;case"ShiftLeft":return 340;case"ControlLeft":return 341;case"AltLeft":return 342;case"OSLeft":return 343;case"ShiftRight":return 344;case"ControlRight":return 345;case"AltRight":return 346;case"OSRight":return 347;case"ContextMenu":return 348}console.log("Unsupported keyboard key: ",e)}function dpi_scale(){return high_dpi&&window.devicePixelRatio||1}function texture_size(e,r,t){return e==gl.ALPHA?r*t:e==gl.RGB?r*t*3:e==gl.RGBA?r*t*4:r*t*3}function mouse_relative_position(e,r){var t=canvas.getBoundingClientRect(),n=(e-t.left)*dpi_scale(),a=(r-t.top)*dpi_scale();return{x:n,y:a}}var emscripten_shaders_hack=!1,importObject={env:{console_debug:function(e){console.debug(UTF8ToString(e))},console_log:function(e){console.log(UTF8ToString(e))},console_info:function(e){console.info(UTF8ToString(e))},console_warn:function(e){console.warn(UTF8ToString(e))},console_error:function(e){console.error(UTF8ToString(e))},set_emscripten_shader_hack:function(e){emscripten_shaders_hack=e},sapp_set_clipboard:function(e,r){clipboard=UTF8ToString(e,r)},dpi_scale,rand:function(){return Math.floor(2147483647*Math.random())},now:function(){return Date.now()/1e3},canvas_width:function(){return Math.floor(canvas.width)},canvas_height:function(){return Math.floor(canvas.height)},glClearDepthf:function(e){gl.clearDepth(e)},glClearColor:function(e,r,t,n){gl.clearColor(e,r,t,n)},glClearStencil:function(e){gl.clearStencil(e)},glColorMask:function(e,r,t,n){gl.colorMask(e,r,t,n)},glScissor:function(e,r,t,n){gl.scissor(e,r,t,n)},glClear:function(e){gl.clear(e)},glGenTextures:function(e,r){_glGenObject(e,r,"createTexture",GL.textures,"glGenTextures")},glActiveTexture:function(e){gl.activeTexture(e)},glBindTexture:function(e,r){GL.validateGLObjectID(GL.textures,r,"glBindTexture","texture"),gl.bindTexture(e,GL.textures[r])},glTexImage2D:function(e,r,t,n,a,o,i,s,u){gl.texImage2D(e,r,t,n,a,o,i,s,u?getArray(u,Uint8Array,texture_size(t,n,a)):null)},glTexSubImage2D:function(e,r,t,n,a,o,i,s,u){gl.texSubImage2D(e,r,t,n,a,o,i,s,u?getArray(u,Uint8Array,texture_size(i,a,o)):null)},glReadPixels:function(e,r,t,n,a,o,i){var s=getArray(i,Uint8Array,texture_size(a,t,n));gl.readPixels(e,r,t,n,a,o,s)},glTexParameteri:function(e,r,t){gl.texParameteri(e,r,t)},glUniform1fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform1fv","location"),assert((3&t)==0,"Pointer to float data passed to glUniform1fv must be aligned to four bytes!");var n=getArray(t,Float32Array,1*r);gl.uniform1fv(GL.uniforms[e],n)},glUniform2fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform2fv","location"),assert((3&t)==0,"Pointer to float data passed to glUniform2fv must be aligned to four bytes!");var n=getArray(t,Float32Array,2*r);gl.uniform2fv(GL.uniforms[e],n)},glUniform3fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform3fv","location"),assert((3&t)==0,"Pointer to float data passed to glUniform3fv must be aligned to four bytes!");var n=getArray(t,Float32Array,3*r);gl.uniform3fv(GL.uniforms[e],n)},glUniform4fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform4fv","location"),assert((3&t)==0,"Pointer to float data passed to glUniform4fv must be aligned to four bytes!");var n=getArray(t,Float32Array,4*r);gl.uniform4fv(GL.uniforms[e],n)},glUniform1iv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform1fv","location"),assert((3&t)==0,"Pointer to i32 data passed to glUniform1iv must be aligned to four bytes!");var n=getArray(t,Int32Array,1*r);gl.uniform1iv(GL.uniforms[e],n)},glUniform2iv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform2fv","location"),assert((3&t)==0,"Pointer to i32 data passed to glUniform2iv must be aligned to four bytes!");var n=getArray(t,Int32Array,2*r);gl.uniform2iv(GL.uniforms[e],n)},glUniform3iv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform3fv","location"),assert((3&t)==0,"Pointer to i32 data passed to glUniform3iv must be aligned to four bytes!");var n=getArray(t,Int32Array,3*r);gl.uniform3iv(GL.uniforms[e],n)},glUniform4iv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform4fv","location"),assert((3&t)==0,"Pointer to i32 data passed to glUniform4iv must be aligned to four bytes!");var n=getArray(t,Int32Array,4*r);gl.uniform4iv(GL.uniforms[e],n)},glBlendFunc:function(e,r){gl.blendFunc(e,r)},glBlendEquationSeparate:function(e,r){gl.blendEquationSeparate(e,r)},glDisable:function(e){gl.disable(e)},glDrawElements:function(e,r,t,n){gl.drawElements(e,r,t,n)},glGetIntegerv:function(e,r){_webglGet(e,r,"EM_FUNC_SIG_PARAM_I")},glUniform1f:function(e,r){GL.validateGLObjectID(GL.uniforms,e,"glUniform1f","location"),gl.uniform1f(GL.uniforms[e],r)},glUniform1i:function(e,r){GL.validateGLObjectID(GL.uniforms,e,"glUniform1i","location"),gl.uniform1i(GL.uniforms[e],r)},glGetAttribLocation:function(e,r){return gl.getAttribLocation(GL.programs[e],UTF8ToString(r))},glEnableVertexAttribArray:function(e){gl.enableVertexAttribArray(e)},glDisableVertexAttribArray:function(e){gl.disableVertexAttribArray(e)},glVertexAttribPointer:function(e,r,t,n,a,o){gl.vertexAttribPointer(e,r,t,!!n,a,o)},glGetUniformLocation:function(e,r){GL.validateGLObjectID(GL.programs,e,"glGetUniformLocation","program");var t=0;if("]"==(r=UTF8ToString(r))[r.length-1]){var n=r.lastIndexOf("[");t="]"!=r[n+1]?parseInt(r.slice(n+1)):0,r=r.slice(0,n)}var a=GL.programInfos[e]&&GL.programInfos[e].uniforms[r];return a&&t>=0&&t<a[0]?a[1]+t:-1},glUniformMatrix4fv:function(e,r,t,n){GL.validateGLObjectID(GL.uniforms,e,"glUniformMatrix4fv","location"),assert((3&n)==0,"Pointer to float data passed to glUniformMatrix4fv must be aligned to four bytes!");var a=getArray(n,Float32Array,16);gl.uniformMatrix4fv(GL.uniforms[e],!!t,a)},glUseProgram:function(e){GL.validateGLObjectID(GL.programs,e,"glUseProgram","program"),gl.useProgram(GL.programs[e])},glGenVertexArrays:function(e,r){_glGenObject(e,r,"createVertexArray",GL.vaos,"glGenVertexArrays")},glGenFramebuffers:function(e,r){_glGenObject(e,r,"createFramebuffer",GL.framebuffers,"glGenFramebuffers")},glBindVertexArray:function(e){gl.bindVertexArray(GL.vaos[e])},glBindFramebuffer:function(e,r){GL.validateGLObjectID(GL.framebuffers,r,"glBindFramebuffer","framebuffer"),gl.bindFramebuffer(e,GL.framebuffers[r])},glGenBuffers:function(e,r){_glGenObject(e,r,"createBuffer",GL.buffers,"glGenBuffers")},glBindBuffer:function(e,r){GL.validateGLObjectID(GL.buffers,r,"glBindBuffer","buffer"),gl.bindBuffer(e,GL.buffers[r])},glBufferData:function(e,r,t,n){gl.bufferData(e,t?getArray(t,Uint8Array,r):r,n)},glBufferSubData:function(e,r,t,n){gl.bufferSubData(e,r,n?getArray(n,Uint8Array,t):t)},glEnable:function(e){gl.enable(e)},glFlush:function(){gl.flush()},glFinish:function(){gl.finish()},glDepthFunc:function(e){gl.depthFunc(e)},glBlendFuncSeparate:function(e,r,t,n){gl.blendFuncSeparate(e,r,t,n)},glViewport:function(e,r,t,n){gl.viewport(e,r,t,n)},glDrawArrays:function(e,r,t){gl.drawArrays(e,r,t)},glCreateProgram:function(){var e=GL.getNewId(GL.programs),r=gl.createProgram();return r.name=e,GL.programs[e]=r,e},glAttachShader:function(e,r){GL.validateGLObjectID(GL.programs,e,"glAttachShader","program"),GL.validateGLObjectID(GL.shaders,r,"glAttachShader","shader"),gl.attachShader(GL.programs[e],GL.shaders[r])},glLinkProgram:function(e){GL.validateGLObjectID(GL.programs,e,"glLinkProgram","program"),gl.linkProgram(GL.programs[e]),GL.populateUniformTable(e)},glPixelStorei:function(e,r){gl.pixelStorei(e,r)},glFramebufferTexture2D:function(e,r,t,n,a){GL.validateGLObjectID(GL.textures,n,"glFramebufferTexture2D","texture"),gl.framebufferTexture2D(e,r,t,GL.textures[n],a)},glGetProgramiv:function(e,r,t){if(assert(t),GL.validateGLObjectID(GL.programs,e,"glGetProgramiv","program"),e>=GL.counter){console.error("GL_INVALID_VALUE in glGetProgramiv");return}if(!GL.programInfos[e]){console.error("GL_INVALID_OPERATION in glGetProgramiv(program="+e+", pname="+r+", p=0x"+t.toString(16)+"): The specified GL object name does not refer to a program object!");return}if(35716==r){var n=gl.getProgramInfoLog(GL.programs[e]);assert(null!==n),getArray(t,Int32Array,1)[0]=n.length+1}else if(35719==r){console.error("unsupported operation");return}else if(35722==r){console.error("unsupported operation");return}else if(35381==r){console.error("unsupported operation");return}else getArray(t,Int32Array,1)[0]=gl.getProgramParameter(GL.programs[e],r)},glCreateShader:function(e){var r=GL.getNewId(GL.shaders);return GL.shaders[r]=gl.createShader(e),r},glStencilFuncSeparate:function(e,r,t,n){gl.stencilFuncSeparate(e,r,t,n)},glStencilMaskSeparate:function(e,r){gl.stencilMaskSeparate(e,r)},glStencilOpSeparate:function(e,r,t,n){gl.stencilOpSeparate(e,r,t,n)},glFrontFace:function(e){gl.frontFace(e)},glCullFace:function(e){gl.cullFace(e)},glCopyTexImage2D:function(e,r,t,n,a,o,i,s){gl.copyTexImage2D(e,r,t,n,a,o,i,s)},glShaderSource:function(e,r,t,n){GL.validateGLObjectID(GL.shaders,e,"glShaderSource","shader");var a=GL.getSource(e,r,t,n);if(emscripten_shaders_hack){a=(a=a.replace(/#extension GL_OES_standard_derivatives : enable/g,"")).replace(/#extension GL_EXT_shader_texture_lod : enable/g,"");var o="";-1!=a.indexOf("gl_FragColor")&&(o+="out mediump vec4 GL_FragColor;\n",a=a.replace(/gl_FragColor/g,"GL_FragColor")),a=(a=(a=(a=(a=(a=(a=(a=(a=(a=(a=(a=-1!=a.indexOf("attribute")?(a=a.replace(/attribute/g,"in")).replace(/varying/g,"out"):a.replace(/varying/g,"in")).replace(/textureCubeLodEXT/g,"textureCubeLod")).replace(/texture2DLodEXT/g,"texture2DLod")).replace(/texture2DProjLodEXT/g,"texture2DProjLod")).replace(/texture2DGradEXT/g,"texture2DGrad")).replace(/texture2DProjGradEXT/g,"texture2DProjGrad")).replace(/textureCubeGradEXT/g,"textureCubeGrad")).replace(/textureCube/g,"texture")).replace(/texture1D/g,"texture")).replace(/texture2D/g,"texture")).replace(/texture3D/g,"texture")).replace(/#version 100/g,"#version 300 es\n"+o)}gl.shaderSource(GL.shaders[e],a)},glGetProgramInfoLog:function(e,r,t,n){GL.validateGLObjectID(GL.programs,e,"glGetProgramInfoLog","program");var a=gl.getProgramInfoLog(GL.programs[e]);assert(null!==a);let o=getArray(n,Uint8Array,r);for(var i=0;i<r;i++)o[i]=a.charCodeAt(i)},glGetString:function(e){var r=gl.getParameter(e).toString(),t=r.length+1,n=wasm_exports.allocate_vec_u8(t),a=new Uint8Array(wasm_memory.buffer,n,t);return a[r.length]=0,stringToUTF8(r,a,0,t),n},glCompileShader:function(e,r,t,n){GL.validateGLObjectID(GL.shaders,e,"glCompileShader","shader"),gl.compileShader(GL.shaders[e])},glGetShaderiv:function(e,r,t){if(assert(t),GL.validateGLObjectID(GL.shaders,e,"glGetShaderiv","shader"),35716==r){var n=gl.getShaderInfoLog(GL.shaders[e]);assert(null!==n),getArray(t,Int32Array,1)[0]=n.length+1}else if(35720==r){var a=gl.getShaderSource(GL.shaders[e]),o=null===a||0==a.length?0:a.length+1;getArray(t,Int32Array,1)[0]=o}else getArray(t,Int32Array,1)[0]=gl.getShaderParameter(GL.shaders[e],r)},glGetShaderInfoLog:function(e,r,t,n){GL.validateGLObjectID(GL.shaders,e,"glGetShaderInfoLog","shader");var a=gl.getShaderInfoLog(GL.shaders[e]);assert(null!==a);let o=getArray(n,Uint8Array,r);for(var i=0;i<r;i++)o[i]=a.charCodeAt(i)},glVertexAttribDivisor:function(e,r){gl.vertexAttribDivisor(e,r)},glDrawArraysInstanced:function(e,r,t,n){gl.drawArraysInstanced(e,r,t,n)},glDrawElementsInstanced:function(e,r,t,n,a){gl.drawElementsInstanced(e,r,t,n,a)},glDeleteShader:function(e){gl.deleteShader(e)},glDeleteBuffers:function(e,r){for(var t=0;t<e;t++){var n=getArray(r+4*t,Uint32Array,1)[0],a=GL.buffers[n];a&&(gl.deleteBuffer(a),a.name=0,GL.buffers[n]=null)}},glDeleteFramebuffers:function(e,r){for(var t=0;t<e;t++){var n=getArray(r+4*t,Uint32Array,1)[0],a=GL.framebuffers[n];a&&(gl.deleteFramebuffer(a),a.name=0,GL.framebuffers[n]=null)}},glDeleteTextures:function(e,r){for(var t=0;t<e;t++){var n=getArray(r+4*t,Uint32Array,1)[0],a=GL.textures[n];a&&(gl.deleteTexture(a),a.name=0,GL.textures[n]=null)}},glGenQueries:function(e,r){_glGenObject(e,r,"createQuery",GL.timerQueries,"glGenQueries")},glDeleteQueries:function(e,r){for(var t=0;t<e;t++){var n=getArray(textures+4*t,Uint32Array,1)[0],a=GL.timerQueries[n];a&&(gl.deleteQuery(a),a.name=0,GL.timerQueries[n]=null)}},glBeginQuery:function(e,r){GL.validateGLObjectID(GL.timerQueries,r,"glBeginQuery","id"),gl.beginQuery(e,GL.timerQueries[r])},glEndQuery:function(e){gl.endQuery(e)},glGetQueryObjectiv:function(e,r,t){GL.validateGLObjectID(GL.timerQueries,e,"glGetQueryObjectiv","id");let n=gl.getQueryObject(GL.timerQueries[e],r);getArray(t,Uint32Array,1)[0]=n},glGetQueryObjectui64v:function(e,r,t){GL.validateGLObjectID(GL.timerQueries,e,"glGetQueryObjectui64v","id");let n=gl.getQueryObject(GL.timerQueries[e],r),a=getArray(t,Uint32Array,2);a[0]=n,a[1]=(n-a[0])/4294967296},glGenerateMipmap:function(e){gl.generateMipmap(e)},setup_canvas_size:function(e){window.high_dpi=e,resize(canvas)},run_animation_loop:function(e){canvas.onmousemove=function(e){var r=mouse_relative_position(e.clientX,e.clientY),t=r.x,n=r.y;wasm_exports.mouse_move(Math.floor(t),Math.floor(n)),(0!=e.movementX||0!=e.movementY)&&wasm_exports.raw_mouse_move(Math.floor(e.movementX),Math.floor(e.movementY))},canvas.onmousedown=function(e){var r=mouse_relative_position(e.clientX,e.clientY),t=r.x,n=r.y,a=into_sapp_mousebutton(e.button);wasm_exports.mouse_down(t,n,a)},canvas.addEventListener("wheel",function(e){e.preventDefault(),wasm_exports.mouse_wheel(-e.deltaX,-e.deltaY)}),canvas.onmouseup=function(e){var r=mouse_relative_position(e.clientX,e.clientY),t=r.x,n=r.y,a=into_sapp_mousebutton(e.button);wasm_exports.mouse_up(t,n,a)},canvas.onkeydown=function(e){var r=into_sapp_keycode(e.code);switch(r){case 32:case 262:case 263:case 264:case 265:case 290:case 291:case 292:case 293:case 294:case 295:case 296:case 297:case 298:case 299:case 259:case 258:case 39:case 47:e.preventDefault()}var t=0;e.ctrlKey&&(t|=2),e.shiftKey&&(t|=1),e.altKey&&(t|=4),wasm_exports.key_down(r,t,e.repeat),(32==r||39==r||47==r)&&wasm_exports.key_press(r)},canvas.onkeyup=function(e){var r=into_sapp_keycode(e.code),t=0;e.ctrlKey&&(t|=2),e.shiftKey&&(t|=1),e.altKey&&(t|=4),wasm_exports.key_up(r,t)},canvas.onkeypress=function(e){!1==(261==into_sapp_keycode(e.code)||e.ctrlKey)&&wasm_exports.key_press(e.charCode)},canvas.addEventListener("touchstart",function(e){for(let r of(e.preventDefault(),e.changedTouches)){let t=mouse_relative_position(r.clientX,r.clientY);wasm_exports.touch(10,r.identifier,t.x,t.y)}}),canvas.addEventListener("touchend",function(e){for(let r of(e.preventDefault(),e.changedTouches)){let t=mouse_relative_position(r.clientX,r.clientY);wasm_exports.touch(12,r.identifier,t.x,t.y)}}),canvas.addEventListener("touchcancel",function(e){for(let r of(e.preventDefault(),e.changedTouches)){let t=mouse_relative_position(r.clientX,r.clientY);wasm_exports.touch(13,r.identifier,t.x,t.y)}}),canvas.addEventListener("touchmove",function(e){for(let r of(e.preventDefault(),e.changedTouches)){let t=mouse_relative_position(r.clientX,r.clientY);wasm_exports.touch(11,r.identifier,t.x,t.y)}}),window.onresize=function(){resize(canvas,wasm_exports.resize)},window.addEventListener("copy",function(e){null!=clipboard&&(event.clipboardData.setData("text/plain",clipboard),event.preventDefault())}),window.addEventListener("cut",function(e){null!=clipboard&&(event.clipboardData.setData("text/plain",clipboard),event.preventDefault())}),window.addEventListener("paste",function(e){e.stopPropagation(),e.preventDefault();var r=(e.clipboardData||window.clipboardData).getData("Text");if(void 0!=r&&null!=r&&0!=r.length){var t=new TextEncoder().encode(r).length,n=wasm_exports.allocate_vec_u8(t),a=new Uint8Array(wasm_memory.buffer,n,t);stringToUTF8(r,a,0,t),wasm_exports.on_clipboard_paste(n,t)}}),window.ondragover=function(e){e.preventDefault()},window.ondrop=async function(e){for(let r of(e.preventDefault(),wasm_exports.on_files_dropped_start(),e.dataTransfer.files)){let t=r.name.length,n=wasm_exports.allocate_vec_u8(t),a=new Uint8Array(wasm_memory.buffer,n,t);stringToUTF8(r.name,a,0,t);let o=await r.arrayBuffer(),i=o.byteLength,s=wasm_exports.allocate_vec_u8(i),u=new Uint8Array(wasm_memory.buffer,s,i);u.set(new Uint8Array(o),0),wasm_exports.on_file_dropped(n,t,s,i)}wasm_exports.on_files_dropped_finish()};let r=document.hasFocus();var t=function(){let e=document.hasFocus();r==e&&(wasm_exports.focus(e),r=e)};document.addEventListener("visibilitychange",t),window.addEventListener("focus",t),window.addEventListener("blur",t),window.requestAnimationFrame(animation)},fs_load_file:function(e,r){var t=UTF8ToString(e,r),n=FS.unique_id;FS.unique_id+=1;var a=new XMLHttpRequest;return a.open("GET",t,!0),a.responseType="arraybuffer",a.onreadystatechange=function(){if(4===this.readyState){if(200===this.status){var e=new Uint8Array(this.response);FS.loaded_files[n]=e,wasm_exports.file_loaded(n)}else FS.loaded_files[n]=null,wasm_exports.file_loaded(n)}},a.send(),n},fs_get_buffer_size:function(e){return null==FS.loaded_files[e]?-1:FS.loaded_files[e].length},fs_take_buffer:function(e,r,t){var n=FS.loaded_files[e];console.assert(n.length<=t);for(var a=new Uint8Array(wasm_memory.buffer,r,t),o=0;o<n.length;o++)a[o]=n[o];delete FS.loaded_files[e]},sapp_set_cursor_grab:function(e){e?canvas.requestPointerLock():document.exitPointerLock()},sapp_set_cursor:function(e,r){canvas.style.cursor=UTF8ToString(e,r)},sapp_is_fullscreen:function(){let e=document.fullscreenElement;return null!=e&&e.id==canvas.id},sapp_set_fullscreen:function(e){e?canvas.requestFullscreen():document.exitFullscreen()},sapp_set_window_size:function(e,r){canvas.width=e,canvas.height=r,resize(canvas,wasm_exports.resize)}}};function register_plugins(e){if(void 0!=e)for(var r=0;r<e.length;r++)void 0!=e[r].register_plugin&&null!=e[r].register_plugin&&e[r].register_plugin(importObject)}function u32_to_semver(e){return(e>>24&255)+"."+(e>>16&255)+"."+(65535&e)}function init_plugins(e){if(void 0!=e)for(var r=0;r<e.length;r++)if(void 0!=e[r].on_init&&null!=e[r].on_init&&e[r].on_init(),void 0==e[r].name||null==e[r].name||void 0==e[r].version||null==e[r].version)console.warn("Some of the registred plugins do not have name or version"),console.warn("Probably old version of the plugin used");else{var t=e[r].name+"_crate_version";if(void 0==wasm_exports[t])console.log("Plugin "+e[r].name+" is present in JS bundle, but is not used in the rust code.");else{var n=u32_to_semver(wasm_exports[t]());e[r].version!=n&&console.error("Plugin "+e[r].name+" version mismatchjs version: "+e[r].version+", crate version: "+n)}}}function miniquad_add_plugin(e){plugins.push(e)}function add_missing_functions_stabs(e){var r=WebAssembly.Module.imports(e);for(let t in r)void 0==importObject.env[r[t].name]&&(console.warn("No "+r[t].name+" function in gl.js"),importObject.env[r[t].name]=function(){console.warn("Missed function: "+r[t].name)})}function load(e){var r=fetch(e);register_plugins(plugins),"function"==typeof WebAssembly.compileStreaming?WebAssembly.compileStreaming(r).then(e=>(add_missing_functions_stabs(e),WebAssembly.instantiate(e,importObject))).then(e=>{wasm_memory=e.exports.memory;var r=u32_to_semver((wasm_exports=e.exports).crate_version());version!=r&&console.error("Version mismatch: gl.js version is: "+version+", miniquad crate version is: "+r),init_plugins(plugins),e.exports.main()}).catch(e=>{console.error("WASM failed to load, probably incompatible gl.js version"),console.error(e)}):r.then(function(e){return e.arrayBuffer()}).then(function(e){return WebAssembly.compile(e)}).then(function(e){return add_missing_functions_stabs(e),WebAssembly.instantiate(e,importObject)}).then(function(e){wasm_memory=e.exports.memory;var r=u32_to_semver((wasm_exports=e.exports).crate_version());version!=r&&console.error("Version mismatch: gl.js version is: "+version+", rust sapp-wasm crate version is: "+r),init_plugins(plugins),e.exports.main()}).catch(e=>{console.error("WASM failed to load, probably incompatible gl.js version"),console.error(e)})}