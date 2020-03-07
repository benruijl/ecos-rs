pub type pfloat = f64;
pub type idxint = i64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct spmat {
    pub jc: *mut idxint,
    pub ir: *mut idxint,
    pub pr: *mut pfloat,
    pub n: idxint,
    pub m: idxint,
    pub nnz: idxint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct expcone {
    pub colstart: [idxint; 3usize],
    pub v: [pfloat; 6usize],
    pub g: [pfloat; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lpcone {
    pub p: idxint,
    pub w: *mut pfloat,
    pub v: *mut pfloat,
    pub kkt_idx: *mut idxint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct socone {
    pub p: idxint,
    pub skbar: *mut pfloat,
    pub zkbar: *mut pfloat,
    pub a: pfloat,
    pub d1: pfloat,
    pub w: pfloat,
    pub eta: pfloat,
    pub eta_square: pfloat,
    pub q: *mut pfloat,
    pub Didx: *mut idxint,
    pub u0: pfloat,
    pub u1: pfloat,
    pub v1: pfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cone {
    pub lpc: *mut lpcone,
    pub soc: *mut socone,
    pub nsoc: idxint,
    pub expc: *mut expcone,
    pub nexc: idxint,
    pub fexv: idxint,
}

extern "C" {
    pub fn getSOCDetails(
        soc: *mut socone,
        conesize: *mut idxint,
        eta_square: *mut pfloat,
        d1: *mut pfloat,
        u0: *mut pfloat,
        u1: *mut pfloat,
        v1: *mut pfloat,
        q: *mut *mut pfloat,
    );

    pub fn unstretch(
        n: idxint,
        p: idxint,
        C: *mut cone,
        Pinv: *mut idxint,
        Px: *mut pfloat,
        dx: *mut pfloat,
        dy: *mut pfloat,
        dz: *mut pfloat,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kkt {
    pub PKPt: *mut spmat,
    pub L: *mut spmat,
    pub D: *mut pfloat,
    pub work1: *mut pfloat,
    pub work2: *mut pfloat,
    pub work3: *mut pfloat,
    pub work4: *mut pfloat,
    pub work5: *mut pfloat,
    pub work6: *mut pfloat,
    pub RHS1: *mut pfloat,
    pub RHS2: *mut pfloat,
    pub dx1: *mut pfloat,
    pub dx2: *mut pfloat,
    pub dy1: *mut pfloat,
    pub dy2: *mut pfloat,
    pub dz1: *mut pfloat,
    pub dz2: *mut pfloat,
    pub P: *mut idxint,
    pub Pinv: *mut idxint,
    pub PK: *mut idxint,
    pub Parent: *mut idxint,
    pub Sign: *mut idxint,
    pub Pattern: *mut idxint,
    pub Flag: *mut idxint,
    pub Lnz: *mut idxint,
    pub delta: pfloat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct settings {
    pub gamma: pfloat,
    pub delta: pfloat,
    pub eps: pfloat,
    pub feastol: pfloat,
    pub abstol: pfloat,
    pub reltol: pfloat,
    pub feastol_inacc: pfloat,
    pub abstol_inacc: pfloat,
    pub reltol_inacc: pfloat,
    pub nitref: idxint,
    pub maxit: idxint,
    pub verbose: idxint,
    pub max_bk_iter: idxint,
    pub bk_scale: pfloat,
    pub centrality: pfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stats {
    pub pcost: pfloat,
    pub dcost: pfloat,
    pub pres: pfloat,
    pub dres: pfloat,
    pub pinf: pfloat,
    pub dinf: pfloat,
    pub pinfres: pfloat,
    pub dinfres: pfloat,
    pub gap: pfloat,
    pub relgap: pfloat,
    pub sigma: pfloat,
    pub mu: pfloat,
    pub step: pfloat,
    pub step_aff: pfloat,
    pub kapovert: pfloat,
    pub iter: idxint,
    pub nitref1: idxint,
    pub nitref2: idxint,
    pub nitref3: idxint,
    pub tsetup: pfloat,
    pub tsolve: pfloat,
    pub pob: idxint,
    pub cb: idxint,
    pub cob: idxint,
    pub pb: idxint,
    pub db: idxint,
    pub affBack: idxint,
    pub cmbBack: idxint,
    pub centrality: pfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pwork {
    pub n: idxint,
    pub m: idxint,
    pub p: idxint,
    pub D: idxint,
    pub x: *mut pfloat,
    pub y: *mut pfloat,
    pub z: *mut pfloat,
    pub s: *mut pfloat,
    pub lambda: *mut pfloat,
    pub kap: pfloat,
    pub tau: pfloat,
    pub best_x: *mut pfloat,
    pub best_y: *mut pfloat,
    pub best_z: *mut pfloat,
    pub best_s: *mut pfloat,
    pub best_kap: pfloat,
    pub best_tau: pfloat,
    pub best_cx: pfloat,
    pub best_by: pfloat,
    pub best_hz: pfloat,
    pub best_info: *mut stats,
    pub dsaff: *mut pfloat,
    pub dzaff: *mut pfloat,
    pub W_times_dzaff: *mut pfloat,
    pub dsaff_by_W: *mut pfloat,
    pub saff: *mut pfloat,
    pub zaff: *mut pfloat,
    pub C: *mut cone,
    pub A: *mut spmat,
    pub G: *mut spmat,
    pub c: *mut pfloat,
    pub b: *mut pfloat,
    pub h: *mut pfloat,
    pub AtoK: *mut idxint,
    pub GtoK: *mut idxint,
    pub xequil: *mut pfloat,
    pub Aequil: *mut pfloat,
    pub Gequil: *mut pfloat,
    pub resx0: pfloat,
    pub resy0: pfloat,
    pub resz0: pfloat,
    pub rx: *mut pfloat,
    pub ry: *mut pfloat,
    pub rz: *mut pfloat,
    pub rt: pfloat,
    pub hresx: pfloat,
    pub hresy: pfloat,
    pub hresz: pfloat,
    pub nx: pfloat,
    pub ny: pfloat,
    pub nz: pfloat,
    pub ns: pfloat,
    pub cx: pfloat,
    pub by: pfloat,
    pub hz: pfloat,
    pub sz: pfloat,
    pub KKT: *mut kkt,
    pub info: *mut stats,
    pub stgs: *mut settings,
}

extern "C" {
    pub fn ECOS_setup(
        n: idxint,
        m: idxint,
        p: idxint,
        l: idxint,
        ncones: idxint,
        q: *mut idxint,
        nex: idxint,
        Gpr: *mut pfloat,
        Gjc: *mut idxint,
        Gir: *mut idxint,
        Apr: *mut pfloat,
        Ajc: *mut idxint,
        Air: *mut idxint,
        c: *mut pfloat,
        h: *mut pfloat,
        b: *mut pfloat,
    ) -> *mut pwork;

    pub fn expConeLineSearch(w: *mut pwork, dtau: pfloat, dkappa: pfloat, affine: idxint)
        -> pfloat;

    pub fn ECOS_solve(w: *mut pwork) -> idxint;

    #[doc = " Cleanup: free memory (not used for embedded solvers, only standalone)"]
    #[doc = ""]
    #[doc = " Use the second argument to give the number of variables to NOT free."]
    #[doc = " This is useful if you want to use the result of the optimization without"]
    #[doc = " copying over the arrays. One use case is the MEX interface, where we"]
    #[doc = " do not want to free x,y,s,z (depending on the number of LHS)."]
    pub fn ECOS_cleanup(w: *mut pwork, keepvars: idxint);

    #[doc = " Version: returns the current version number"]
    #[doc = " Use a character array of length 7 to obtain the version number"]
    #[doc = " in the format"]
    #[doc = "      x.y.zzz"]
    #[doc = " where x is the major, y the minor and zzz the build number"]
    pub fn ECOS_ver() -> *const ::std::os::raw::c_char;

    pub fn ecos_updateDataEntry_h(w: *mut pwork, idx: idxint, value: pfloat);

    pub fn ecos_updateDataEntry_c(w: *mut pwork, idx: idxint, value: pfloat);

    pub fn ECOS_updateData(
        w: *mut pwork,
        Gpr: *mut pfloat,
        Apr: *mut pfloat,
        c: *mut pfloat,
        h: *mut pfloat,
        b: *mut pfloat,
    );
}
