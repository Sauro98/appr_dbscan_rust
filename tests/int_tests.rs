extern crate appr_dbscan;
use appr_dbscan::do_appr_dbscan_auto_dimensionality_file;
use appr_dbscan::do_appr_dbscan_file;
use appr_dbscan::utils::DBSCANResult;
#[test]
fn out_3_test_1() {
    let res : DBSCANResult<3> = do_appr_dbscan_file("./datasets/out_3.txt", 1.25, 1.0, 15);
    assert_eq!(res.len() - 1, 11);
    assert_eq!(res[0].len(),1476);
}

#[test]
fn out_3_test_2() {
    let res : DBSCANResult<3> = do_appr_dbscan_file("./datasets/out_3.txt", 1.25, 0.7, 15);
    assert_eq!(res.len() - 1, 18);
    assert_eq!(res[0].len(),2038);
}

#[test]
fn out_3_test_3() {
    let res : DBSCANResult<3> = do_appr_dbscan_file("./datasets/out_3.txt", 1.25, 0.001, 15);
    assert_eq!(res.len() - 1, 33);
    assert_eq!(res[0].len(),3162);
}


#[test]
fn out20000_test_1() {
    let (res,dim) = do_appr_dbscan_auto_dimensionality_file("./datasets/out20000.txt", 0.3, 0.1, 30);
    assert_eq!(dim,2);
    assert_eq!(res.len() - 1, 4);
    assert_eq!(res[0].len(),0);
}

#[test]
fn out20000_test_2() {
    let (res,_) = do_appr_dbscan_auto_dimensionality_file("./datasets/out20000.txt", 0.25, 0.001, 30);
    assert_eq!(res.len() - 1, 6);
    assert_eq!(res[0].len(),0);
}

#[test]
fn out20000_test_3() {
    let (res,_) = do_appr_dbscan_auto_dimensionality_file("./datasets/out20000.txt", 0.05, 0.0001, 15);
    assert_eq!(res.len() - 1, 8);
    assert_eq!(res[0].len(),305);
}