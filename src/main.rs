
slint::slint! {
  struct CadreData {
    live: int
  }

  component Cadre inherits Rectangle {
    in-out property <int> live;
    height: 25px;
    width: 25px;

    Rectangle {
      background: live==1 ? #fff : #000;
      animate background { duration: 500ms; }
      border-width: 0.5px;
      border-color: live==1 ? #000 : #fff;
    }
  }

  export component App inherits Window {
    width: 625px;
    height: 700px;
    callback play();

    Rectangle {
      x: 312px;
      y: 640px;
      width: 50px;
      height: 50px;
      background: ta.has-hover ? #333 : #fff;
      animate background { duration: 100ms; }
      border-width: 1px;
      border-color: ta.has-hover ? #fff : #333;
      Text {
        text: "INIT";
        font-size: 16px;
        color: #000;
      }
      ta := TouchArea { clicked => { root.play() } }
    }

    in property <[CadreData]> cadres;
    for cad[i] in cadres : Cadre := Cadre {
      x: mod(i, 25) * 25px;
      y: floor(i / 25) * 25px;
      width: 25px;
      height: 25px;
      live: cad.live;
    }
  }
}

fn concat_2d_vectors<T>(vectors: Vec<Vec<T>>) -> Vec<T> {
  let mut concatenated: Vec<T> = Vec::new();
  for vector in vectors {
    concatenated.extend(vector);
  }
  concatenated
}

fn has_at_least_one_one(matrix: &Vec<Vec<i32>>) -> bool {
  for row in matrix.iter() {
    for &value in row.iter() {
      if value == 1 {
        return true;
      }
    }
  }
  return false;
}

fn main() {
  let app = App::new().unwrap();
  let mut cadres: Vec<Vec<CadreData>> = Vec::new();
  let mut state: Vec<Vec<i32>> = Vec::new();

  for _ in 0..25 {
    let mut row: Vec<CadreData> = Vec::new();
    let mut row_state: Vec<i32> = Vec::new();
    for _ in 0..25 {
      row.push(CadreData { live: 0 });
      row_state.push(0); 
    }
    cadres.push(row);
    state.push(row_state);
  }

  for i in 0..25 {
    cadres[0][i].live = 1;
  }

  use rand::seq::SliceRandom;
  let mut rng = rand::thread_rng();
  let mut vec_unit = concat_2d_vectors(cadres.clone());
  vec_unit.shuffle(&mut rng);

  for i in 0..25 {
    for j in 0..25 {
      state[i][j] = vec_unit[i+j].live;
      cadres[i][j].live = vec_unit[i+j].live
    }
  }

  let cadres_model = std::rc::Rc::new(slint::VecModel::from(vec_unit.clone()));
  app.set_cadres(cadres_model.clone().into());

  const SIZE: usize = 25;
  let app_weak = app.as_weak();
  let mut weight: i32 = 0;
  let mut i_minus_1=0;
  let mut j_minus_1=0;
  app_weak.unwrap().on_play(move || {
    if !has_at_least_one_one(&state) {
      for i in 0..25 {
        cadres[0][i].live = 1;
      }
      vec_unit = concat_2d_vectors(cadres.clone());
      vec_unit.shuffle(&mut rng);
      for i in 0..25 {
        for j in 0..25 {
          state[i][j] = vec_unit[i+j].live;
          cadres[i][j].live = vec_unit[i+j].live
        }
      }
      let cadres_model = std::rc::Rc::new(slint::VecModel::from(vec_unit.clone()));
      app_weak.unwrap().set_cadres(cadres_model.clone().into());
    } else {
      let mut new_state = state.clone();
      for (i, row) in cadres.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
          i_minus_1 = if i == 0 { SIZE - 1 } else { i };
          j_minus_1 = if j == 0 { SIZE - 1 } else { j };
  
          weight += state[(i_minus_1-1) % SIZE][(j_minus_1-1) % SIZE];
          weight += state[(i) % SIZE][(j_minus_1-1) % SIZE];
          weight += state[(i+1) % SIZE][(j_minus_1-1) % SIZE];
          weight += state[(i_minus_1-1) % SIZE][(j) % SIZE];
          weight += state[(i+1) % SIZE][(j) % SIZE];
          weight += state[(i_minus_1-1) % SIZE][(j+1) % SIZE];
          weight += state[(i) % SIZE][(j+1) % SIZE];
          weight += state[(i+1) % SIZE][(j+1) % SIZE];
  
          if state[i][j]==0 && weight==3 {
            new_state[i][j] = 1;
          } else if state[i][j]==1 && (weight<2 || weight>3) {
            new_state[i][j] = 0;
          }
          weight = 0;
        }
      }
      for i in 0..25 {
        for j in 0..25 {
          state[i][j] = new_state[i][j];
          cadres[i][j].live = new_state[i][j]
        }
      }
      let vec_unit = concat_2d_vectors(cadres.clone());
      let cadres_model = std::rc::Rc::new(slint::VecModel::from(vec_unit));
      app_weak.unwrap().set_cadres(cadres_model.clone().into());
    }
  });

  app.run().unwrap();
}