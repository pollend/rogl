use crate::gl;
use crate::gl::feature::EntryGLFn;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
pub trait GL13 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glReadPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry().glReadPixels)(_x, _y, _width, _height, _format, _type, _pixels)
    }
    unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glNormal3fv(&self, _v: *const GLfloat) {
        (self.entry().glNormal3fv)(_v)
    }
    unsafe fn glTexGend(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLdouble,
    ) {
        (self.entry().glTexGend)(_coord, _pname, _param)
    }
    unsafe fn glTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLdouble,
    ) {
        (self.entry().glTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glGetMapdv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLdouble) {
        (self.entry().glGetMapdv)(_target, _query, _v)
    }
    unsafe fn glMultiTexCoord3iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord3iv)(_target, _v)
    }
    unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {
        (self.entry().glReadBuffer)(_src)
    }
    unsafe fn glColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glColor3uiv)(_v)
    }
    unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {
        (self.entry().glEvalCoord2f)(_u, _v)
    }
    unsafe fn glMultiTexCoord2i(&self, _target: TextureUnit, _s: GLint, _t: GLint) {
        (self.entry().glMultiTexCoord2i)(_target, _s, _t)
    }
    unsafe fn glCompressedTexSubImage3D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _zoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _imageSize, _data,
        )
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor3iv(&self, _v: *const GLint) {
        (self.entry().glColor3iv)(_v)
    }
    unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glRasterPos3i)(_x, _y, _z)
    }
    unsafe fn glVertex2dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex2dv)(_v)
    }
    unsafe fn glVertex4fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex4fv)(_v)
    }
    unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glVertex4s)(_x, _y, _z, _w)
    }
    unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadMatrixf)(_m)
    }
    unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {
        (self.entry().glColor4ui)(_red, _green, _blue, _alpha)
    }
    unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glVertex2s)(_x, _y)
    }
    unsafe fn glEvalCoord1f(&self, _u: GLfloat) {
        (self.entry().glEvalCoord1f)(_u)
    }
    unsafe fn glFrustum(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
        (self.entry().glFrustum)(_left, _right, _bottom, _top, _zNear, _zFar)
    }
    unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        (self.entry().glDeleteTextures)(_n, _textures)
    }
    unsafe fn glBegin(&self, _mode: PrimitiveType) {
        (self.entry().glBegin)(_mode)
    }
    unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {
        (self.entry().glSelectBuffer)(_size, _buffer)
    }
    unsafe fn glLoadName(&self, _name: GLuint) {
        (self.entry().glLoadName)(_name)
    }
    unsafe fn glGetClipPlane(&self, _plane: ClipPlaneName, _equation: *mut GLdouble) {
        (self.entry().glGetClipPlane)(_plane, _equation)
    }
    unsafe fn glMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLfloat,
    ) {
        (self.entry().glMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord1fv)(_v)
    }
    unsafe fn glIndexiv(&self, _c: *const GLint) {
        (self.entry().glIndexiv)(_c)
    }
    unsafe fn glPixelTransferi(&self, _pname: PixelTransferParameter, _param: GLint) {
        (self.entry().glPixelTransferi)(_pname, _param)
    }
    unsafe fn glTexSubImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _type, _pixels,
        )
    }
    unsafe fn glInterleavedArrays(
        &self,
        _format: InterleavedArrayFormat,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glInterleavedArrays)(_format, _stride, _pointer)
    }
    unsafe fn glGetCompressedTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetCompressedTexImage)(_target, _level, _img)
    }
    unsafe fn glColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glColor3bv)(_v)
    }
    unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadMatrixd)(_m)
    }
    unsafe fn glPixelTransferf(&self, _pname: PixelTransferParameter, _param: GLfloat) {
        (self.entry().glPixelTransferf)(_pname, _param)
    }
    unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {
        (self.entry().glEdgeFlagv)(_flag)
    }
    unsafe fn glNormal3bv(&self, _v: *const GLbyte) {
        (self.entry().glNormal3bv)(_v)
    }
    unsafe fn glTexEnvf(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLfloat,
    ) {
        (self.entry().glTexEnvf)(_target, _pname, _param)
    }
    unsafe fn glColor4f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glColor4f)(_red, _green, _blue, _alpha)
    }
    unsafe fn glLightfv(&self, _light: LightName, _pname: LightParameter, _params: *const GLfloat) {
        (self.entry().glLightfv)(_light, _pname, _params)
    }
    unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord2dv)(_u)
    }
    unsafe fn glGetMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertex3f)(_x, _y, _z)
    }
    unsafe fn glTexSubImage3D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _zoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format, _type,
            _pixels,
        )
    }
    unsafe fn glGetFloatv(&self, _pname: GetPName, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glOrtho(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
        (self.entry().glOrtho)(_left, _right, _bottom, _top, _zNear, _zFar)
    }
    unsafe fn glVertex3sv(&self, _v: *const GLshort) {
        (self.entry().glVertex3sv)(_v)
    }
    unsafe fn glGetLightfv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetLightfv)(_light, _pname, _params)
    }
    unsafe fn glCompressedTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexImage1D)(
            _target,
            _level,
            _internalformat,
            _width,
            _border,
            _imageSize,
            _data,
        )
    }
    unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glColor3ub)(_red, _green, _blue)
    }
    unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glColor3s)(_red, _green, _blue)
    }
    unsafe fn glMultiTexCoord4sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord4sv)(_target, _v)
    }
    unsafe fn glTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLfloat,
    ) {
        (self.entry().glTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glLoadTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadTransposeMatrixf)(_m)
    }
    unsafe fn glGetPixelMapusv(&self, _map: PixelMap, _values: *mut GLushort) {
        (self.entry().glGetPixelMapusv)(_map, _values)
    }
    unsafe fn glPopClientAttrib(&self) {
        (self.entry().glPopClientAttrib)()
    }
    unsafe fn glDrawArrays(&self, _mode: PrimitiveType, _first: GLint, _count: GLsizei) {
        (self.entry().glDrawArrays)(_mode, _first, _count)
    }
    unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glColor3us)(_red, _green, _blue)
    }
    unsafe fn glCompressedTexImage3D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexImage3D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _depth,
            _border,
            _imageSize,
            _data,
        )
    }
    unsafe fn glDisableClientState(&self, _array: EnableCap) {
        (self.entry().glDisableClientState)(_array)
    }
    unsafe fn glAreTexturesResident(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _residences: *mut GLboolean,
    ) -> GLboolean {
        (self.entry().glAreTexturesResident)(_n, _textures, _residences)
    }
    unsafe fn glMap1d(
        &self,
        _target: MapTarget,
        _u1: GLdouble,
        _u2: GLdouble,
        _stride: GLint,
        _order: GLint,
        _points: *const GLdouble,
    ) {
        (self.entry().glMap1d)(_target, _u1, _u2, _stride, _order, _points)
    }
    unsafe fn glColor3sv(&self, _v: *const GLshort) {
        (self.entry().glColor3sv)(_v)
    }
    unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
    }
    unsafe fn glColor4iv(&self, _v: *const GLint) {
        (self.entry().glColor4iv)(_v)
    }
    unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos3sv)(_v)
    }
    unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {
        (self.entry().glRectsv)(_v1, _v2)
    }
    unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {
        (self.entry().glRectd)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
        (self.entry().glTexCoord4s)(_s, _t, _r, _q)
    }
    unsafe fn glRasterPos2iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos2iv)(_v)
    }
    unsafe fn glColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glColor3fv)(_v)
    }
    unsafe fn glIndexi(&self, _c: GLint) {
        (self.entry().glIndexi)(_c)
    }
    unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRotated)(_angle, _x, _y, _z)
    }
    unsafe fn glDrawRangeElements(
        &self,
        _mode: PrimitiveType,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawRangeElements)(_mode, _start, _end, _count, _type, _indices)
    }
    unsafe fn glMap2d(
        &self,
        _target: MapTarget,
        _u1: GLdouble,
        _u2: GLdouble,
        _ustride: GLint,
        _uorder: GLint,
        _v1: GLdouble,
        _v2: GLdouble,
        _vstride: GLint,
        _vorder: GLint,
        _points: *const GLdouble,
    ) {
        (self.entry().glMap2d)(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        )
    }
    unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {
        (self.entry().glPolygonStipple)(_mask)
    }
    unsafe fn glPushClientAttrib(&self, _mask: ClientAttribMask) {
        (self.entry().glPushClientAttrib)(_mask)
    }
    unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {
        (self.entry().glNormal3i)(_nx, _ny, _nz)
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glVertex4f)(_x, _y, _z, _w)
    }
    unsafe fn glFogi(&self, _pname: FogParameter, _param: GLint) {
        (self.entry().glFogi)(_pname, _param)
    }
    unsafe fn glLightModelf(&self, _pname: LightModelParameter, _param: GLfloat) {
        (self.entry().glLightModelf)(_pname, _param)
    }
    unsafe fn glTexGenf(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLfloat,
    ) {
        (self.entry().glTexGenf)(_coord, _pname, _param)
    }
    unsafe fn glTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLfloat,
    ) {
        (self.entry().glTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glPixelMapfv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLfloat) {
        (self.entry().glPixelMapfv)(_map, _mapsize, _values)
    }
    unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {
        (self.entry().glEvalCoord2d)(_u, _v)
    }
    unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultMatrixf)(_m)
    }
    unsafe fn glGetTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLdouble,
    ) {
        (self.entry().glGetTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glRasterPos2f)(_x, _y)
    }
    unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glTranslatef)(_x, _y, _z)
    }
    unsafe fn glGetString(&self, _name: StringName) -> *const GLubyte {
        (self.entry().glGetString)(_name)
    }
    unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glRasterPos4f)(_x, _y, _z, _w)
    }
    unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
        (self.entry().glTexCoord4d)(_s, _t, _r, _q)
    }
    unsafe fn glVertex4sv(&self, _v: *const GLshort) {
        (self.entry().glVertex4sv)(_v)
    }
    unsafe fn glColorPointer(
        &self,
        _size: GLint,
        _type: ColorPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glColorPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glRasterPos4i)(_x, _y, _z, _w)
    }
    unsafe fn glMultiTexCoord2d(&self, _target: TextureUnit, _s: GLdouble, _t: GLdouble) {
        (self.entry().glMultiTexCoord2d)(_target, _s, _t)
    }
    unsafe fn glMultiTexCoord3i(&self, _target: TextureUnit, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glMultiTexCoord3i)(_target, _s, _t, _r)
    }
    unsafe fn glColor4ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor4ubv)(_v)
    }
    unsafe fn glEnableClientState(&self, _array: EnableCap) {
        (self.entry().glEnableClientState)(_array)
    }
    unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord1dv)(_v)
    }
    unsafe fn glMaterialf(&self, _face: MaterialFace, _pname: MaterialParameter, _param: GLfloat) {
        (self.entry().glMaterialf)(_face, _pname, _param)
    }
    unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {
        (self.entry().glColor4b)(_red, _green, _blue, _alpha)
    }
    unsafe fn glTexCoord1f(&self, _s: GLfloat) {
        (self.entry().glTexCoord1f)(_s)
    }
    unsafe fn glGetMapfv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLfloat) {
        (self.entry().glGetMapfv)(_target, _query, _v)
    }
    unsafe fn glActiveTexture(&self, _texture: TextureUnit) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glMultiTexCoord1d(&self, _target: TextureUnit, _s: GLdouble) {
        (self.entry().glMultiTexCoord1d)(_target, _s)
    }
    unsafe fn glMultiTexCoord1f(&self, _target: TextureUnit, _s: GLfloat) {
        (self.entry().glMultiTexCoord1f)(_target, _s)
    }
    unsafe fn glMultiTexCoord2iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord2iv)(_target, _v)
    }
    unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertex3s)(_x, _y, _z)
    }
    unsafe fn glPixelMapusv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLushort) {
        (self.entry().glPixelMapusv)(_map, _mapsize, _values)
    }
    unsafe fn glMultiTexCoord3sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord3sv)(_target, _v)
    }
    unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {
        (self.entry().glTexCoord2f)(_s, _t)
    }
    unsafe fn glMultiTexCoord4iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord4iv)(_target, _v)
    }
    unsafe fn glCopyTexSubImage3D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _zoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glCopyTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _x, _y, _width, _height,
        )
    }
    unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord2dv)(_v)
    }
    unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRange)(_n, _f)
    }
    unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos2dv)(_v)
    }
    unsafe fn glColor4sv(&self, _v: *const GLshort) {
        (self.entry().glColor4sv)(_v)
    }
    unsafe fn glIndexd(&self, _c: GLdouble) {
        (self.entry().glIndexd)(_c)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glCullFace(&self, _mode: CullFaceMode) {
        (self.entry().glCullFace)(_mode)
    }
    unsafe fn glColorMaterial(&self, _face: MaterialFace, _mode: ColorMaterialParameter) {
        (self.entry().glColorMaterial)(_face, _mode)
    }
    unsafe fn glTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage1D)(_target, _level, _xoffset, _width, _format, _type, _pixels)
    }
    unsafe fn glPrioritizeTextures(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _priorities: *const GLfloat,
    ) {
        (self.entry().glPrioritizeTextures)(_n, _textures, _priorities)
    }
    unsafe fn glMultiTexCoord1i(&self, _target: TextureUnit, _s: GLint) {
        (self.entry().glMultiTexCoord1i)(_target, _s)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLint,
    ) {
        (self.entry().glMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glColor3usv(&self, _v: *const GLushort) {
        (self.entry().glColor3usv)(_v)
    }
    unsafe fn glGetLightiv(&self, _light: LightName, _pname: LightParameter, _params: *mut GLint) {
        (self.entry().glGetLightiv)(_light, _pname, _params)
    }
    unsafe fn glMultTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultTransposeMatrixf)(_m)
    }
    unsafe fn glPushAttrib(&self, _mask: AttribMask) {
        (self.entry().glPushAttrib)(_mask)
    }
    unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRasterPos3d)(_x, _y, _z)
    }
    unsafe fn glListBase(&self, _base: GLuint) {
        (self.entry().glListBase)(_base)
    }
    unsafe fn glVertex3iv(&self, _v: *const GLint) {
        (self.entry().glVertex3iv)(_v)
    }
    unsafe fn glEvalMesh1(&self, _mode: MeshMode1, _i1: GLint, _i2: GLint) {
        (self.entry().glEvalMesh1)(_mode, _i1, _i2)
    }
    unsafe fn glIndexMask(&self, _mask: GLuint) {
        (self.entry().glIndexMask)(_mask)
    }
    unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glTexCoord3d)(_s, _t, _r)
    }
    unsafe fn glIndexubv(&self, _c: *const GLubyte) {
        (self.entry().glIndexubv)(_c)
    }
    unsafe fn glMultiTexCoord2sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord2sv)(_target, _v)
    }
    unsafe fn glMultiTexCoord3s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
    ) {
        (self.entry().glMultiTexCoord3s)(_target, _s, _t, _r)
    }
    unsafe fn glTexParameteri(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
        (self.entry().glTexParameteri)(_target, _pname, _param)
    }
    unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glColor3d)(_red, _green, _blue)
    }
    unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glRasterPos2s)(_x, _y)
    }
    unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glRasterPos3s)(_x, _y, _z)
    }
    unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultMatrixd)(_m)
    }
    unsafe fn glMultiTexCoord4s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
        _q: GLshort,
    ) {
        (self.entry().glMultiTexCoord4s)(_target, _s, _t, _r, _q)
    }
    unsafe fn glColor4d(
        &self,
        _red: GLdouble,
        _green: GLdouble,
        _blue: GLdouble,
        _alpha: GLdouble,
    ) {
        (self.entry().glColor4d)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClientActiveTexture(&self, _texture: TextureUnit) {
        (self.entry().glClientActiveTexture)(_texture)
    }
    unsafe fn glMap2f(
        &self,
        _target: MapTarget,
        _u1: GLfloat,
        _u2: GLfloat,
        _ustride: GLint,
        _uorder: GLint,
        _v1: GLfloat,
        _v2: GLfloat,
        _vstride: GLint,
        _vorder: GLint,
        _points: *const GLfloat,
    ) {
        (self.entry().glMap2f)(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        )
    }
    unsafe fn glRenderMode(&self, _mode: RenderingMode) -> GLint {
        (self.entry().glRenderMode)(_mode)
    }
    unsafe fn glColor4ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte) {
        (self.entry().glColor4ub)(_red, _green, _blue, _alpha)
    }
    unsafe fn glVertex3dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex3dv)(_v)
    }
    unsafe fn glCopyTexImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
    ) {
        (self.entry().glCopyTexImage2D)(
            _target,
            _level,
            _internalformat,
            _x,
            _y,
            _width,
            _height,
            _border,
        )
    }
    unsafe fn glPixelStoref(&self, _pname: PixelStoreParameter, _param: GLfloat) {
        (self.entry().glPixelStoref)(_pname, _param)
    }
    unsafe fn glCopyTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _border: GLint,
    ) {
        (self.entry().glCopyTexImage1D)(_target, _level, _internalformat, _x, _y, _width, _border)
    }
    unsafe fn glMultiTexCoord2fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord2fv)(_target, _v)
    }
    unsafe fn glLightModelfv(&self, _pname: LightModelParameter, _params: *const GLfloat) {
        (self.entry().glLightModelfv)(_pname, _params)
    }
    unsafe fn glTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLint,
    ) {
        (self.entry().glTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glTexCoord2iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord2iv)(_v)
    }
    unsafe fn glMultiTexCoord4i(
        &self,
        _target: TextureUnit,
        _s: GLint,
        _t: GLint,
        _r: GLint,
        _q: GLint,
    ) {
        (self.entry().glMultiTexCoord4i)(_target, _s, _t, _r, _q)
    }
    unsafe fn glBitmap(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _xorig: GLfloat,
        _yorig: GLfloat,
        _xmove: GLfloat,
        _ymove: GLfloat,
        _bitmap: *const GLubyte,
    ) {
        (self.entry().glBitmap)(_width, _height, _xorig, _yorig, _xmove, _ymove, _bitmap)
    }
    unsafe fn glDepthFunc(&self, _func: DepthFunction) {
        (self.entry().glDepthFunc)(_func)
    }
    unsafe fn glEdgeFlag(&self, _flag: GLboolean) {
        (self.entry().glEdgeFlag)(_flag)
    }
    unsafe fn glColorMask(
        &self,
        _red: GLboolean,
        _green: GLboolean,
        _blue: GLboolean,
        _alpha: GLboolean,
    ) {
        (self.entry().glColorMask)(_red, _green, _blue, _alpha)
    }
    unsafe fn glPixelMapuiv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLuint) {
        (self.entry().glPixelMapuiv)(_map, _mapsize, _values)
    }
    unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glTranslated)(_x, _y, _z)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glTexCoord3iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord3iv)(_v)
    }
    unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertex4i)(_x, _y, _z, _w)
    }
    unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {
        (self.entry().glNormal3d)(_nx, _ny, _nz)
    }
    unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glColor3i)(_red, _green, _blue)
    }
    unsafe fn glClear(&self, _mask: ClearBufferMask) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glNormal3sv(&self, _v: *const GLshort) {
        (self.entry().glNormal3sv)(_v)
    }
    unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos2fv)(_v)
    }
    unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
        (self.entry().glTexCoord4i)(_s, _t, _r, _q)
    }
    unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glVertex2i)(_x, _y)
    }
    unsafe fn glTexGeni(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLint,
    ) {
        (self.entry().glTexGeni)(_coord, _pname, _param)
    }
    unsafe fn glTexCoord1i(&self, _s: GLint) {
        (self.entry().glTexCoord1i)(_s)
    }
    unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord4dv)(_v)
    }
    unsafe fn glRasterPos3iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos3iv)(_v)
    }
    unsafe fn glGetPixelMapuiv(&self, _map: PixelMap, _values: *mut GLuint) {
        (self.entry().glGetPixelMapuiv)(_map, _values)
    }
    unsafe fn glMultiTexCoord3d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
    ) {
        (self.entry().glMultiTexCoord3d)(_target, _s, _t, _r)
    }
    unsafe fn glTexParameterf(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
        (self.entry().glTexParameterf)(_target, _pname, _param)
    }
    unsafe fn glFogiv(&self, _pname: FogParameter, _params: *const GLint) {
        (self.entry().glFogiv)(_pname, _params)
    }
    unsafe fn glIndexub(&self, _c: GLubyte) {
        (self.entry().glIndexub)(_c)
    }
    unsafe fn glMultiTexCoord3f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
    ) {
        (self.entry().glMultiTexCoord3f)(_target, _s, _t, _r)
    }
    unsafe fn glLightf(&self, _light: LightName, _pname: LightParameter, _param: GLfloat) {
        (self.entry().glLightf)(_light, _pname, _param)
    }
    unsafe fn glGetTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glGetTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos4fv)(_v)
    }
    unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glRasterPos4d)(_x, _y, _z, _w)
    }
    unsafe fn glCopyTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
    ) {
        (self.entry().glCopyTexSubImage1D)(_target, _level, _xoffset, _x, _y, _width)
    }
    unsafe fn glIndexPointer(
        &self,
        _type: IndexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glIndexPointer)(_type, _stride, _pointer)
    }
    unsafe fn glMultiTexCoord4fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord4fv)(_target, _v)
    }
    unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {
        (self.entry().glRectfv)(_v1, _v2)
    }
    unsafe fn glIndexfv(&self, _c: *const GLfloat) {
        (self.entry().glIndexfv)(_c)
    }
    unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos4sv)(_v)
    }
    unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {
        (self.entry().glTexCoord2s)(_s, _t)
    }
    unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {
        (self.entry().glEvalPoint2)(_i, _j)
    }
    unsafe fn glGetTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord2fv)(_u)
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glCopyTexSubImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glCopyTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _x, _y, _width, _height,
        )
    }
    unsafe fn glGetTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetTexImage)(_target, _level, _format, _type, _pixels)
    }
    unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos3dv)(_v)
    }
    unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {
        (self.entry().glNormal3s)(_nx, _ny, _nz)
    }
    unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glColor3ui)(_red, _green, _blue)
    }
    unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {
        (self.entry().glPixelZoom)(_xfactor, _yfactor)
    }
    unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
    }
    unsafe fn glGetMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glMultiTexCoord1dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord1dv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord2dv)(_target, _v)
    }
    unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord2sv)(_v)
    }
    unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {
        (self.entry().glGetPolygonStipple)(_mask)
    }
    unsafe fn glMultiTexCoord2f(&self, _target: TextureUnit, _s: GLfloat, _t: GLfloat) {
        (self.entry().glMultiTexCoord2f)(_target, _s, _t)
    }
    unsafe fn glTexEnvi(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLint,
    ) {
        (self.entry().glTexEnvi)(_target, _pname, _param)
    }
    unsafe fn glMultiTexCoord4d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
        _q: GLdouble,
    ) {
        (self.entry().glMultiTexCoord4d)(_target, _s, _t, _r, _q)
    }
    unsafe fn glLoadTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadTransposeMatrixd)(_m)
    }
    unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glTexCoord3f)(_s, _t, _r)
    }
    unsafe fn glPushName(&self, _name: GLuint) {
        (self.entry().glPushName)(_name)
    }
    unsafe fn glDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawElements)(_mode, _count, _type, _indices)
    }
    unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {
        (self.entry().glDeleteLists)(_list, _range)
    }
    unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glRasterPos4s)(_x, _y, _z, _w)
    }
    unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord1sv)(_v)
    }
    unsafe fn glTexImage3D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _border: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexImage3D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _depth,
            _border,
            _format,
            _type,
            _pixels,
        )
    }
    unsafe fn glShadeModel(&self, _mode: ShadingModel) {
        (self.entry().glShadeModel)(_mode)
    }
    unsafe fn glIndexs(&self, _c: GLshort) {
        (self.entry().glIndexs)(_c)
    }
    unsafe fn glDisable(&self, _cap: EnableCap) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glTexCoord3i)(_s, _t, _r)
    }
    unsafe fn glClipPlane(&self, _plane: ClipPlaneName, _equation: *const GLdouble) {
        (self.entry().glClipPlane)(_plane, _equation)
    }
    unsafe fn glRasterPos4iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos4iv)(_v)
    }
    unsafe fn glMultiTexCoord2s(&self, _target: TextureUnit, _s: GLshort, _t: GLshort) {
        (self.entry().glMultiTexCoord2s)(_target, _s, _t)
    }
    unsafe fn glTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLint,
    ) {
        (self.entry().glTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {
        (self.entry().glRectdv)(_v1, _v2)
    }
    unsafe fn glPassThrough(&self, _token: GLfloat) {
        (self.entry().glPassThrough)(_token)
    }
    unsafe fn glColor4usv(&self, _v: *const GLushort) {
        (self.entry().glColor4usv)(_v)
    }
    unsafe fn glClearIndex(&self, _c: GLfloat) {
        (self.entry().glClearIndex)(_c)
    }
    unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord1fv)(_u)
    }
    unsafe fn glEvalPoint1(&self, _i: GLint) {
        (self.entry().glEvalPoint1)(_i)
    }
    unsafe fn glGetPixelMapfv(&self, _map: PixelMap, _values: *mut GLfloat) {
        (self.entry().glGetPixelMapfv)(_map, _values)
    }
    unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRasterPos3f)(_x, _y, _z)
    }
    unsafe fn glIndexdv(&self, _c: *const GLdouble) {
        (self.entry().glIndexdv)(_c)
    }
    unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        (self.entry().glTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glTexCoord1d(&self, _s: GLdouble) {
        (self.entry().glTexCoord1d)(_s)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glPolygonMode(&self, _face: MaterialFace, _mode: PolygonMode) {
        (self.entry().glPolygonMode)(_face, _mode)
    }
    unsafe fn glCallLists(
        &self,
        _n: GLsizei,
        _type: ListNameType,
        _lists: *const std::os::raw::c_void,
    ) {
        (self.entry().glCallLists)(_n, _type, _lists)
    }
    unsafe fn glNormal3dv(&self, _v: *const GLdouble) {
        (self.entry().glNormal3dv)(_v)
    }
    unsafe fn glClearAccum(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearAccum)(_red, _green, _blue, _alpha)
    }
    unsafe fn glMap1f(
        &self,
        _target: MapTarget,
        _u1: GLfloat,
        _u2: GLfloat,
        _stride: GLint,
        _order: GLint,
        _points: *const GLfloat,
    ) {
        (self.entry().glMap1f)(_target, _u1, _u2, _stride, _order, _points)
    }
    unsafe fn glStencilMask(&self, _mask: GLuint) {
        (self.entry().glStencilMask)(_mask)
    }
    unsafe fn glMatrixMode(&self, _mode: MatrixMode) {
        (self.entry().glMatrixMode)(_mode)
    }
    unsafe fn glTexCoord4iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord4iv)(_v)
    }
    unsafe fn glInitNames(&self) {
        (self.entry().glInitNames)()
    }
    unsafe fn glBlendFunc(&self, _sfactor: BlendingFactor, _dfactor: BlendingFactor) {
        (self.entry().glBlendFunc)(_sfactor, _dfactor)
    }
    unsafe fn glColor4bv(&self, _v: *const GLbyte) {
        (self.entry().glColor4bv)(_v)
    }
    unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {
        (self.entry().glRectf)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glBindTexture(&self, _target: TextureTarget, _texture: GLuint) {
        (self.entry().glBindTexture)(_target, _texture)
    }
    unsafe fn glEnable(&self, _cap: EnableCap) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexLevelParameteriv)(_target, _level, _pname, _params)
    }
    unsafe fn glGenLists(&self, _range: GLsizei) -> GLuint {
        (self.entry().glGenLists)(_range)
    }
    unsafe fn glPopMatrix(&self) {
        (self.entry().glPopMatrix)()
    }
    unsafe fn glGetTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glTexCoord3s)(_s, _t, _r)
    }
    unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertex3d)(_x, _y, _z)
    }
    unsafe fn glEvalMesh2(&self, _mode: MeshMode2, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
        (self.entry().glEvalMesh2)(_mode, _i1, _i2, _j1, _j2)
    }
    unsafe fn glPushMatrix(&self) {
        (self.entry().glPushMatrix)()
    }
    unsafe fn glVertex4dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex4dv)(_v)
    }
    unsafe fn glColor4uiv(&self, _v: *const GLuint) {
        (self.entry().glColor4uiv)(_v)
    }
    unsafe fn glTexCoordPointer(
        &self,
        _size: GLint,
        _type: TexCoordPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexCoordPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glPopAttrib(&self) {
        (self.entry().glPopAttrib)()
    }
    unsafe fn glCompressedTexImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexImage2D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _border,
            _imageSize,
            _data,
        )
    }
    unsafe fn glCompressedTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage1D)(
            _target, _level, _xoffset, _width, _format, _imageSize, _data,
        )
    }
    unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glRasterPos2d)(_x, _y)
    }
    unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        (self.entry().glNormal3f)(_nx, _ny, _nz)
    }
    unsafe fn glTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _border: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexImage1D)(
            _target,
            _level,
            _internalformat,
            _width,
            _border,
            _format,
            _type,
            _pixels,
        )
    }
    unsafe fn glFeedbackBuffer(&self, _size: GLsizei, _type: FeedbackType, _buffer: *mut GLfloat) {
        (self.entry().glFeedbackBuffer)(_size, _type, _buffer)
    }
    unsafe fn glIsList(&self, _list: GLuint) -> GLboolean {
        (self.entry().glIsList)(_list)
    }
    unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glScaled)(_x, _y, _z)
    }
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glMultiTexCoord3dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord3dv)(_target, _v)
    }
    unsafe fn glMultTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultTransposeMatrixd)(_m)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glColor4fv(&self, _v: *const GLfloat) {
        (self.entry().glColor4fv)(_v)
    }
    unsafe fn glVertex4iv(&self, _v: *const GLint) {
        (self.entry().glVertex4iv)(_v)
    }
    unsafe fn glTexImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexImage2D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _border,
            _format,
            _type,
            _pixels,
        )
    }
    unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glColor3f)(_red, _green, _blue)
    }
    unsafe fn glEndList(&self) {
        (self.entry().glEndList)()
    }
    unsafe fn glTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLfloat,
    ) {
        (self.entry().glTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glPixelStorei(&self, _pname: PixelStoreParameter, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glGetTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glDrawBuffer(&self, _buf: DrawBufferMode) {
        (self.entry().glDrawBuffer)(_buf)
    }
    unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {
        (self.entry().glTexCoord2d)(_s, _t)
    }
    unsafe fn glAccum(&self, _op: AccumOp, _value: GLfloat) {
        (self.entry().glAccum)(_op, _value)
    }
    unsafe fn glPopName(&self) {
        (self.entry().glPopName)()
    }
    unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRotatef)(_angle, _x, _y, _z)
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glNormal3iv(&self, _v: *const GLint) {
        (self.entry().glNormal3iv)(_v)
    }
    unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertex2d)(_x, _y)
    }
    unsafe fn glVertex2iv(&self, _v: *const GLint) {
        (self.entry().glVertex2iv)(_v)
    }
    unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos4dv)(_v)
    }
    unsafe fn glVertex3fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex3fv)(_v)
    }
    unsafe fn glMultiTexCoord1iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord1iv)(_target, _v)
    }
    unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {
        (self.entry().glColor4i)(_red, _green, _blue, _alpha)
    }
    unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertex2f)(_x, _y)
    }
    unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {
        (self.entry().glMapGrid1d)(_un, _u1, _u2)
    }
    unsafe fn glDrawPixels(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawPixels)(_width, _height, _format, _type, _pixels)
    }
    unsafe fn glVertex2sv(&self, _v: *const GLshort) {
        (self.entry().glVertex2sv)(_v)
    }
    unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glVertex4d)(_x, _y, _z, _w)
    }
    unsafe fn glMultiTexCoord1sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord1sv)(_target, _v)
    }
    unsafe fn glLighti(&self, _light: LightName, _pname: LightParameter, _param: GLint) {
        (self.entry().glLighti)(_light, _pname, _param)
    }
    unsafe fn glEvalCoord1d(&self, _u: GLdouble) {
        (self.entry().glEvalCoord1d)(_u)
    }
    unsafe fn glCopyPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _type: PixelCopyType,
    ) {
        (self.entry().glCopyPixels)(_x, _y, _width, _height, _type)
    }
    unsafe fn glMultiTexCoord4dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord4dv)(_target, _v)
    }
    unsafe fn glColor4s(&self, _red: GLshort, _green: GLshort, _blue: GLshort, _alpha: GLshort) {
        (self.entry().glColor4s)(_red, _green, _blue, _alpha)
    }
    unsafe fn glIndexsv(&self, _c: *const GLshort) {
        (self.entry().glIndexsv)(_c)
    }
    unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord3dv)(_v)
    }
    unsafe fn glHint(&self, _target: HintTarget, _mode: HintMode) {
        (self.entry().glHint)(_target, _mode)
    }
    unsafe fn glLightModeli(&self, _pname: LightModelParameter, _param: GLint) {
        (self.entry().glLightModeli)(_pname, _param)
    }
    unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {
        (self.entry().glLineStipple)(_factor, _pattern)
    }
    unsafe fn glAlphaFunc(&self, _func: AlphaFunction, _ref: GLfloat) {
        (self.entry().glAlphaFunc)(_func, _ref)
    }
    unsafe fn glGetMapiv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLint) {
        (self.entry().glGetMapiv)(_target, _query, _v)
    }
    unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {
        (self.entry().glRecti)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glTexCoord1iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord1iv)(_v)
    }
    unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord2fv)(_v)
    }
    unsafe fn glArrayElement(&self, _i: GLint) {
        (self.entry().glArrayElement)(_i)
    }
    unsafe fn glColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glColor3dv)(_v)
    }
    unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos3fv)(_v)
    }
    unsafe fn glFogf(&self, _pname: FogParameter, _param: GLfloat) {
        (self.entry().glFogf)(_pname, _param)
    }
    unsafe fn glCallList(&self, _list: GLuint) {
        (self.entry().glCallList)(_list)
    }
    unsafe fn glColor4us(
        &self,
        _red: GLushort,
        _green: GLushort,
        _blue: GLushort,
        _alpha: GLushort,
    ) {
        (self.entry().glColor4us)(_red, _green, _blue, _alpha)
    }
    unsafe fn glMultiTexCoord3fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord3fv)(_target, _v)
    }
    unsafe fn glMateriali(&self, _face: MaterialFace, _pname: MaterialParameter, _param: GLint) {
        (self.entry().glMateriali)(_face, _pname, _param)
    }
    unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {
        (self.entry().glMapGrid1f)(_un, _u1, _u2)
    }
    unsafe fn glCompressedTexSubImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _imageSize, _data,
        )
    }
    unsafe fn glNormalPointer(
        &self,
        _type: NormalPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glNormalPointer)(_type, _stride, _pointer)
    }
    unsafe fn glMultiTexCoord1s(&self, _target: TextureUnit, _s: GLshort) {
        (self.entry().glMultiTexCoord1s)(_target, _s)
    }
    unsafe fn glColor4dv(&self, _v: *const GLdouble) {
        (self.entry().glColor4dv)(_v)
    }
    unsafe fn glTexCoord1s(&self, _s: GLshort) {
        (self.entry().glTexCoord1s)(_s)
    }
    unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glColor3b)(_red, _green, _blue)
    }
    unsafe fn glLoadIdentity(&self) {
        (self.entry().glLoadIdentity)()
    }
    unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord4fv)(_v)
    }
    unsafe fn glGetPointerv(
        &self,
        _pname: GetPointervPName,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetPointerv)(_pname, _params)
    }
    unsafe fn glLightiv(&self, _light: LightName, _pname: LightParameter, _params: *const GLint) {
        (self.entry().glLightiv)(_light, _pname, _params)
    }
    unsafe fn glMapGrid2d(
        &self,
        _un: GLint,
        _u1: GLdouble,
        _u2: GLdouble,
        _vn: GLint,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
        (self.entry().glMapGrid2d)(_un, _u1, _u2, _vn, _v1, _v2)
    }
    unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos2sv)(_v)
    }
    unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertex3i)(_x, _y, _z)
    }
    unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
        (self.entry().glTexCoord4f)(_s, _t, _r, _q)
    }
    unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glScalef)(_x, _y, _z)
    }
    unsafe fn glMultiTexCoord1fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord1fv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
        _q: GLfloat,
    ) {
        (self.entry().glMultiTexCoord4f)(_target, _s, _t, _r, _q)
    }
    unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
        (self.entry().glNormal3b)(_nx, _ny, _nz)
    }
    unsafe fn glIndexf(&self, _c: GLfloat) {
        (self.entry().glIndexf)(_c)
    }
    unsafe fn glVertexPointer(
        &self,
        _size: GLint,
        _type: VertexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glLightModeliv(&self, _pname: LightModelParameter, _params: *const GLint) {
        (self.entry().glLightModeliv)(_pname, _params)
    }
    unsafe fn glNewList(&self, _list: GLuint, _mode: ListMode) {
        (self.entry().glNewList)(_list, _mode)
    }
    unsafe fn glLogicOp(&self, _opcode: LogicOp) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glVertex2fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex2fv)(_v)
    }
    unsafe fn glPointSize(&self, _size: GLfloat) {
        (self.entry().glPointSize)(_size)
    }
    unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {
        (self.entry().glRects)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor3ubv)(_v)
    }
    unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord1dv)(_u)
    }
    unsafe fn glGetTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glEdgeFlagPointer(&self, _stride: GLsizei, _pointer: *const std::os::raw::c_void) {
        (self.entry().glEdgeFlagPointer)(_stride, _pointer)
    }
    unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glRasterPos2i)(_x, _y)
    }
    unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord4sv)(_v)
    }
    unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord3sv)(_v)
    }
    unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord3fv)(_v)
    }
    unsafe fn glFogfv(&self, _pname: FogParameter, _params: *const GLfloat) {
        (self.entry().glFogfv)(_pname, _params)
    }
    unsafe fn glMapGrid2f(
        &self,
        _un: GLint,
        _u1: GLfloat,
        _u2: GLfloat,
        _vn: GLint,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
        (self.entry().glMapGrid2f)(_un, _u1, _u2, _vn, _v1, _v2)
    }
    unsafe fn glIsEnabled(&self, _cap: EnableCap) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
    }
    unsafe fn glGenTextures(&self, _n: GLsizei, _textures: *mut GLuint) {
        (self.entry().glGenTextures)(_n, _textures)
    }
    unsafe fn glIsTexture(&self, _texture: GLuint) -> GLboolean {
        (self.entry().glIsTexture)(_texture)
    }
    unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {
        (self.entry().glTexCoord2i)(_s, _t)
    }
    unsafe fn glGetTexLevelParameterfv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexLevelParameterfv)(_target, _level, _pname, _params)
    }
    unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {
        (self.entry().glRectiv)(_v1, _v2)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
    unsafe fn glEnd(&self) {
        (self.entry().glEnd)()
    }
    unsafe fn glDepthMask(&self, _flag: GLboolean) {
        (self.entry().glDepthMask)(_flag)
    }
}