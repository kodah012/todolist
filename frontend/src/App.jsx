import { useEffect, useState } from 'react';
import "./App.css";

const AVATAR_1 = "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar_lf92vl.png";
const AVATAR_2 = "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar2_erqray.png";

function App() {
  const [people, setPeople] = useState([]);

  useEffect(() => {
    fetch("http://0.0.0.0:3000/people")
      .then(res => res.json())
      .then(people => setPeople(people));
  });
  
  return (
    <div>
      {people.map((person, index) => (
        <div key={index} className="card">
          <img src={index % 2 == 0 ? AVATAR_1 : AVATAR_2} alt="Avatar" />
          <div className="container">
            <h4>
              <b>{person.name}</b>
            </h4>
            <p>Age: {person.age}</p>
            <p>Favorite Food: {person.favorite_food ?? "Unknown"}</p>
          </div>
        </div>
      ))}
    </div>
  );
}

export default App;